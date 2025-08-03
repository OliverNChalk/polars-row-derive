use polars_row_derive::IterToDataFrame;
use test_private::IterToDataFrameDynTestStructCrate;
use test_private::IterToDataFrameTestStructCrate;

#[derive(IterToDataFrame)]
pub struct TestStruct {
    a: i32,
    b: i32,
}

#[test]
fn works_exact() {
    let df = (0..10)
        .map(|i| TestStruct { a: i, b: i })
        .to_dataframe()
        .unwrap();
    assert_eq!(df.shape(), (10, 2));
}

#[test]
fn works_dyn() {
    let df = (0..10)
        .filter(|i| i % 2 == 0)
        .map(|i| TestStruct { a: i, b: i })
        .to_dataframe_dyn()
        .unwrap();
    assert_eq!(df.shape(), (5, 2));
}

mod test_private {
    use polars_row_derive::IterToDataFrame;

    #[derive(IterToDataFrame)]
    pub(crate) struct TestStructCrate {
        pub(crate) a: i32,
        pub(crate) b: i32,
    }
}

#[test]
fn works_exact_crate() {
    let df = (0..10)
        .map(|i| test_private::TestStructCrate { a: i, b: i })
        .to_dataframe()
        .unwrap();
    assert_eq!(df.shape(), (10, 2));
}

#[test]
fn works_dyn_crate() {
    let df = (0..10)
        .filter(|i| i % 2 == 0)
        .map(|i| test_private::TestStructCrate { a: i, b: i })
        .to_dataframe_dyn()
        .unwrap();
    assert_eq!(df.shape(), (5, 2));
}
