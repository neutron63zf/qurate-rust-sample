// Graph, ClassicPure, Qisikit, ClassicDensityなどの実装
pub trait QBackend {}

#[derive(Debug)]
// 測定結果
pub enum QComputationalBasis {
    Zero,
    One,
}

// グラフに関する操作（単一Qubit）
pub trait QPublicGraph<T>
where
    T: QBackend,
{
    // 単一Qubitの測定を行う
    fn measure(&self) -> QComputationalBasis;
}

// Qubit
pub trait Qubit<T>
where
    T: QBackend,
{
    // 単一Qubitの計算依存グラフを作成する
    fn inspect(&self) -> Box<dyn QPublicGraph<T>>;
    // 単一Qubitだけでなく、複数のQubitの計算依存グラフをまとめて作成する（共通する依存関係を適切に処理する）
    fn associate(&self, qs: Vec<Box<&dyn Qubit<T>>>) -> Vec<Box<dyn QPublicGraph<T>>>;
}

// 量子ゲート（測定も含む）
pub trait QGate<T>
where
    T: QBackend,
{
    // 量子ゲートの入力
    type Input;
    // 量子ゲートの出力
    type Output;
    // 量子ゲートの入力から出力の構築
    fn apply(qs: Self::Input) -> Self::Output;
}
