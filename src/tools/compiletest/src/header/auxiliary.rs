//! Code for dealing with test directives that request an "auxiliary" crate to
//! be built and made available to the test in some way.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::{fs, io, iter};
use std::path::Path;

use crate::common::Config;
use crate::header::directives::{AUX_BIN, AUX_BUILD, AUX_CODEGEN_BACKEND, AUX_CRATE, PROC_MACRO};
use crate::header::DirectiveLine;

use super::iter_header;

/// Properties parsed from `aux-*` test directives.
#[derive(Clone, Debug, Default)]
pub(crate) struct AuxProps {
    /// Other crates that should be built and made available to this test.
    /// These are filenames relative to `./auxiliary/` in the test's directory.
    pub(crate) builds: Vec<String>,
    /// Auxiliary crates that should be compiled as `#![crate_type = "bin"]`.
    pub(crate) bins: Vec<String>,
    /// Similar to `builds`, but a list of NAME=somelib.rs of dependencies
    /// to build and pass with the `--extern` flag.
    pub(crate) crates: Vec<(String, String)>,
    /// Same as `builds`, but for proc-macros.
    pub(crate) proc_macros: Vec<String>,
    /// Similar to `builds`, but also uses the resulting dylib as a
    /// `-Zcodegen-backend` when compiling the test file.
    pub(crate) codegen_backend: Option<String>,
}

impl AuxProps {
    /// Yields all of the paths (relative to `./auxiliary/`) that have been
    /// specified in `aux-*` directives for this test.
    pub(crate) fn all_aux_path_strings(&self) -> impl Iterator<Item = &str> {
        let Self { builds, bins, crates, proc_macros, codegen_backend } = self;

        iter::empty()
            .chain(builds.iter().map(String::as_str))
            .chain(bins.iter().map(String::as_str))
            .chain(crates.iter().map(|(_, path)| path.as_str()))
            .chain(proc_macros.iter().map(String::as_str))
            .chain(codegen_backend.iter().map(String::as_str))
    }
}

/// If the given test directive line contains an `aux-*` directive, parse it
/// and update [`AuxProps`] accordingly.
pub(super) fn parse_and_update_aux(config: &Config, ln: &str, aux: &mut AuxProps) {
    // println!("parse_and_update_aux");
    if !(ln.starts_with("aux-") || ln.starts_with("proc-macro")) {
        return;
    }

    config.push_name_value_directive(ln, AUX_BUILD, &mut aux.builds, |r| r.trim().to_string());
    config.push_name_value_directive(ln, AUX_BIN, &mut aux.bins, |r| r.trim().to_string());
    config.push_name_value_directive(ln, AUX_CRATE, &mut aux.crates, parse_aux_crate);
    config
        .push_name_value_directive(ln, PROC_MACRO, &mut aux.proc_macros, |r| r.trim().to_string());
    if let Some(r) = config.parse_name_value_directive(ln, AUX_CODEGEN_BACKEND) {
        aux.codegen_backend = Some(r.trim().to_owned());
    }
}

fn parse_aux_crate(r: String) -> (String, String) {
    let mut parts = r.trim().splitn(2, '=');
    (
        parts.next().expect("missing aux-crate name (e.g. log=log.rs)").to_string(),
        parts.next().expect("missing aux-crate value (e.g. log=log.rs)").to_string(),
    )
}

pub(crate) fn check_cycles(config: &Config, dir: &Path) -> bool {
    let mut vertices = vec![];
    let mut edges= HashMap::new();

    match build_graph(config, dir, &mut vertices, &mut edges) {
        Ok(_) => {},
        Err(_) => panic!(""),
    };

    if edges.len() > 0 {
        println!("====================================");
        println!("{:?}", dir);
        println!("{:?}", vertices);
        println!("{:?}", edges);
        println!("====================================");    
    }

    has_cycle(&vertices, &edges)
}

fn build_graph(config: &Config, dir: &Path, vertices: &mut Vec<String>, edges: &mut HashMap<String, Vec<String>>) -> io::Result<()> {
    for file in fs::read_dir(dir)? {
        let file = file?;
        let file_path = file.path();


        if file_path.is_dir() {
            match build_graph(config, &file_path, vertices, edges) {
                Ok(_) => {},
                Err(_) => panic!(""),
            };
        } else {
            vertices.push(file.file_name().into_string().unwrap());
            let mut aux = AuxProps::default();
            let mut poisoned = false;
            let f = File::open(file_path).expect("open test file to parse earlyprops");
            iter_header(
                config.mode,
                &config.suite,
                &mut poisoned,
                &file.path(),
                f,
                &mut |DirectiveLine { raw_directive: ln, .. }| {
                    parse_and_update_aux(config, ln, &mut aux);
                },
            );

            let s = file.file_name().into_string().unwrap();
            let mut v = vec![];
            for to in aux.all_aux_path_strings() {
                v.push(to.to_string());
            }

            if v.len() > 0 {
                edges.insert(s, v);
            }
        }
    }

    Ok(())
}

fn has_cycle(vertices: &Vec<String>, edges: &HashMap<String, Vec<String>>) -> bool {
    let mut checked = HashSet::with_capacity(vertices.len());
    let mut on_search = HashSet::with_capacity(4);
    let mut path = Vec::new();

    for vertex in vertices.iter() {
        if !checked.contains(vertex) {
            if search(vertices, edges, &vertex, &mut checked, &mut on_search, &mut path) {
                return true;
            }
        }
    }

    fn search(
        vertices: &Vec<String>,
        edges: &HashMap<String, Vec<String>>,
        vertex: &str,
        checked: &mut HashSet<String>,
        on_search: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        if !on_search.insert(vertex.to_string()) {
            let mut cyclic_path = vec![vertex];
            for v in path.iter().rev() {
                if v == vertex {
                    break;
                }

                cyclic_path.push(v);
            }

            println!("detect!!!!!!!! 1 {:?}", cyclic_path);

            return true;
        }

        if checked.insert(vertex.to_string()) {
            path.push(vertex.to_string());
            if let Some(e) = edges.get(&vertex.to_string()) {
                for to in e.iter() {
                    if search(vertices, edges, &to, checked, on_search, path) {
                        println!("detect!!!!!!!!!2 {}, {:?} {:?}", vertex, vertices, edges);
                        return true;
                    }
                }    
            }
        }

        on_search.remove(&vertex.to_string());
        false
    }

    false
}