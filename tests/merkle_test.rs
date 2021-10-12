use sha3::Sha3_256;
use bs3::backend::MemoryBackend;
use bs3::merkle::{append_only, Merkle};
use bs3::OperationBytes;
use bs3::Result;

#[test]
fn test() -> Result<()>{
    let mut s = MemoryBackend::new();
    let mut merkle = append_only::AppendOnlyMerkle::<Sha3_256>::new("merkle_test",0);

    let batch = vec![
        (1_i32.to_be_bytes().to_vec(),OperationBytes::Update("11".as_bytes().to_vec())),
        (2_i32.to_be_bytes().to_vec(),OperationBytes::Update("22".as_bytes().to_vec())),
        (3_i32.to_be_bytes().to_vec(),OperationBytes::Update("33".as_bytes().to_vec())),
        (4_i32.to_be_bytes().to_vec(),OperationBytes::Update("44".as_bytes().to_vec())),
        (5_i32.to_be_bytes().to_vec(),OperationBytes::Update("55".as_bytes().to_vec())),
    ];

    merkle.insert(&mut s,batch.as_slice())?;
    let root = &merkle.root(&s)?[..];
    assert_eq!(&[50, 158, 207, 250, 183, 91, 122, 244, 170, 118, 205, 214, 22, 159, 21, 53, 248, 75, 99, 251, 207, 139, 58, 0, 45, 71, 143, 237, 227, 41, 140, 127],root);

    let batch = vec![
        (6_i32.to_be_bytes().to_vec(),OperationBytes::Update("66".as_bytes().to_vec())),
        (7_i32.to_be_bytes().to_vec(),OperationBytes::Update("77".as_bytes().to_vec())),
        (8_i32.to_be_bytes().to_vec(),OperationBytes::Update("88".as_bytes().to_vec())),
        (9_i32.to_be_bytes().to_vec(),OperationBytes::Update("99".as_bytes().to_vec())),
        (10_i32.to_be_bytes().to_vec(),OperationBytes::Update("1010".as_bytes().to_vec())),
    ];

    merkle.insert(&mut s,batch.as_slice())?;
    let root = &merkle.root(&s)?[..];
    assert_eq!(&[143, 41, 145, 99, 124, 242, 46, 6, 33, 71, 214, 45, 79, 94, 244, 44, 87, 245, 122, 116, 216, 100, 66, 26, 130, 153, 242, 159, 7, 102, 33, 65],root);
    Ok(())
}