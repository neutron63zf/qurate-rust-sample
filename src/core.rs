use std::fmt::Debug;
use std::rc::Rc;

// Graph, ClassicPure, Qisikit, ClassicDensityなどの実装
pub trait QBackend {}

#[derive(Debug)]
// 測定結果
pub enum QBasis {
    Zero,
    One,
}

// グラフに関する操作
pub trait QPublicGraph<T>: Debug
where
    T: QBackend,
{
    // 単一Qubitの測定を行う
    fn measure(&self) -> QBasis;
}

pub trait QInspectable<T>: Debug
where
    T: QBackend,
{
    // 単一の計算依存グラフを作成する
    fn inspect(&self) -> Rc<dyn QPublicGraph<T>>;
}

pub trait QMultipleInspectable<T>: Debug
where
    T: QBackend,
{
    fn inspect_vec(qs: Vec<Box<&dyn Qubit<T>>>) -> Vec<Box<dyn QPublicGraph<T>>>;
}

// Qubit
pub trait Qubit<T>: QInspectable<T>
where
    T: QBackend,
{
}

// 量子ゲート（測定も含む）
pub trait QGate<T>: QInspectable<T>
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
