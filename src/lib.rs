// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.1.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

// Routing sims library.

// Actual simulations are examples.

#![feature(inclusive_range_syntax)]
#![allow(non_snake_case)]   // this is maths

extern crate rand;

pub mod prob;
pub mod sim;


// We could use templating but there's no reason not to do the easy thing and
// fix types.

pub type NN = u64;
pub type RR = f64;


pub trait Quorum {
    /// Get the number of messages needed for quorum. If the quorum algorithm
    /// does anything more complicated (e.g. check node age) then this should
    /// return `None`.
    fn quorum_size(&self) -> Option<NN>;
    /// Set the quorum size. If `quorum_size()` does not return `None`, then it
    /// should return the number last set by this method; otherwise the action
    /// taken by this method is up to the implementation.
    fn set_quorum_size(&mut self, n: NN);
}


pub trait SimTool {
    /// Get the total number of nodes
    fn total_nodes(&self) -> NN;
    /// Set the total number of nodes
    fn set_total_nodes(&mut self, n: NN);

    /// Get the number of malicious nodes
    fn malicious_nodes(&self) -> NN;
    /// Set the number of malicious nodes
    fn set_malicious_nodes(&mut self, n: NN);

    /// Get the minimum group size
    fn min_group_size(&self) -> NN;
    /// Set the minimum group size
    fn set_min_group_size(&mut self, n: NN);

    /// Get the quorum algorithm
    fn quorum(&self) -> &Quorum;
    /// Adjust the quorum
    fn quorum_mut(&mut self) -> &mut Quorum;

    /// Set whether the probabilities of compromise returned should be from the
    /// point of view of a single group (any=false) or any group within the
    /// entire network (any=true).
    ///
    /// On creation this should be set to false.
    fn set_any(&mut self, any: bool);

    /// Calculate the probability of compromise (range: 0 to 1).
    fn calc_p_compromise(&self) -> RR;
}
