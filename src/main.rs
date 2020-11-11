use qurate::backends::graph::*;
use qurate::core::*;

fn main() {
    // 初期状態を準備（0 -> 2へテレポートする）
    let q0_0 = init_graph_qubit(QBasis::Zero);
    let q1_0 = init_graph_qubit(QBasis::Zero);
    let q2_0 = init_graph_qubit(QBasis::Zero);

    // 1にH
    let q1_1 = GraphHadamard::apply(q1_0);

    // 1,2にCNOT
    let (q1_2, q2_2) = GraphCNOT::apply((q1_1, q2_0));

    // 0,1にCNOT
    let (q0_3, q1_3) = GraphCNOT::apply((q0_0, q1_2));

    // 0にH
    let q0_4 = GraphHadamard::apply(q0_3);

    // 測定フェーズ: 1,2にCNOT
    let (q1_5, q2_5) = GraphCNOT::apply((q1_3, q2_2));

    // 測定フェーズ: 0,2にCZ
    let (q0_6, q2_6) = GraphCZ::apply((q0_4, q2_5));

    // 最後のqubit
    let (q0_7, q1_7, q2_7) = (q0_6, q1_5, q2_6);

    println!("{:#?}", (&q0_7, &q1_7, &q2_7));
}
