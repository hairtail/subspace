use crate::single_plot_farm::SinglePlotPieceGetter;
use crate::ws_rpc_server::PieceGetter;
use subspace_core_primitives::{Piece, PieceIndex, PieceIndexHash};

/// Abstraction that can get pieces out of internal plots
#[derive(Debug, Clone)]
pub struct SingleDiskFarmPieceGetter {
    single_plot_piece_getters: Vec<SinglePlotPieceGetter>,
}

impl SingleDiskFarmPieceGetter {
    /// Create new piece getter for many single plot farms
    pub fn new(single_plot_piece_getters: Vec<SinglePlotPieceGetter>) -> Self {
        Self {
            single_plot_piece_getters,
        }
    }
}

impl PieceGetter for SingleDiskFarmPieceGetter {
    fn get_piece(
        &self,
        piece_index: PieceIndex,
        piece_index_hash: PieceIndexHash,
    ) -> Option<Piece> {
        self.single_plot_piece_getters
            .iter()
            .find_map(|single_plot_piece_getter| {
                single_plot_piece_getter.get_piece(piece_index, piece_index_hash)
            })
    }
}