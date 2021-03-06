// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.

extern crate wfst;

use wfst::semiring::Weight;
use wfst::semiring::floatweight::TropicalWeight;
use wfst::{Fst, MutableFst, ExpandedFst, Arc};
use wfst::wfst_vec::{StdArc, VecFst};
use wfst::algorithms;

fn main() {

    let a = StdArc::new(0, 0, TropicalWeight::<f32>::new(Some(12.0000001)), 1);
    let b = StdArc::new(0, 0, TropicalWeight::<f64>::new(Some(12.0000001)), 1);
    let c = b.clone();

    println!("Weight: {:?}", a.weight());
    println!("Olabel: {:?}", b.olabel());
    println!("Nextstate: {:?}", c.nextstate());
    println!("");

    let mut fst = VecFst::<TropicalWeight<f32>>::new();
    let s0 = fst.add_state(TropicalWeight::<f32>::zero());
    let s1 = fst.add_state(TropicalWeight::<f32>::zero());
    let s2 = fst.add_state(TropicalWeight::<f32>::one());
    fst.set_start(s0);
    fst.add_arc(s0, s1, 0, 0, TropicalWeight::<f32>::zero());
    fst.add_arc(s1, s2, 0, 0, TropicalWeight::<f32>::zero());
    fst.add_arc(s1, s2, 0, 0, TropicalWeight::<f32>::zero());
    println!("{:?}", fst);
    println!("");

    for a in fst.arc_iter(s0) {
        println!("{:?}", a);
        fst.add_state(TropicalWeight::new(Some(23.0)));
    }

    //The following is a no-op since there are no arcs from s2
    for a in fst.arc_iter(s2) {
        println!("Hello {:?}", a);
        fst.add_state(TropicalWeight::new(Some(23.0)));
    }

    println!("");
    println!("{:?}", fst);
    println!("");
    println!("Number of states: {}", fst.get_numstates());    
    println!("==============================");
    println!("");
    fst = algorithms::extendfinal(fst);
    println!("");
    println!("{:?}", fst);    
    println!("");
    println!("Number of states: {}", fst.get_numstates());    
    println!("==============================");
    println!("");
    fst = algorithms::unextendfinal(fst);
    println!("");
    println!("{:?}", fst);    
    println!("");
    println!("Number of states: {}", fst.get_numstates());
    println!("==============================");
    println!("reverse()");
    let revfst: VecFst<_> = algorithms::reverse(fst.clone());
    println!("");
    println!("{:?}", revfst);    
    println!("==============================");
    println!("shortest_distance()");
    let distances = algorithms::shortestpath::shortest_distance(fst.clone());
    println!("");
    println!("{:?}", distances);    
    println!("==============================");
    println!("shortest_paths()");
    let newfst: VecFst<_> = algorithms::shortestpath::shortest_paths(fst, 1, false);
    println!("");
    println!("{:?}", newfst);    

}
