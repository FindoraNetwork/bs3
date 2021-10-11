use sha3::Sha3_256;
use bs3::backend::MemoryBackend;
use bs3::merkle::{append_only, Merkle};
use bs3::merkle_key;
use bs3::OperationBytes;
use bs3::Result;

#[test]
fn test() -> Result<()>{
    let mut s = MemoryBackend::new();
    let mut merkle = append_only::AppendOnlyMerkle::<Sha3_256>::default();

    let prev_key = merkle_key("merkle", 0);
    let cur_key = merkle_key("merkle", 1);

    let batch = vec![
        (1_i32.to_be_bytes().to_vec(),OperationBytes::Update("11".as_bytes().to_vec())),
        (2_i32.to_be_bytes().to_vec(),OperationBytes::Update("22".as_bytes().to_vec())),
        (3_i32.to_be_bytes().to_vec(),OperationBytes::Update("33".as_bytes().to_vec())),
        (4_i32.to_be_bytes().to_vec(),OperationBytes::Update("44".as_bytes().to_vec())),
        (5_i32.to_be_bytes().to_vec(),OperationBytes::Update("55".as_bytes().to_vec())),
    ];

    merkle.insert(prev_key,cur_key.clone(),&mut s,batch.as_slice())?;
    let root = &merkle.root(cur_key,&s)?.unwrap()[..];
    assert_eq!(&[50, 158, 207, 250, 183, 91, 122, 244, 170, 118, 205, 214, 22, 159, 21, 53, 248, 75, 99, 251, 207, 139, 58, 0, 45, 71, 143, 237, 227, 41, 140, 127],root);

    let prev_key = merkle_key("merkle", 1);
    let cur_key = merkle_key("merkle", 2);

    let batch = vec![
        (6_i32.to_be_bytes().to_vec(),OperationBytes::Update("66".as_bytes().to_vec())),
        (7_i32.to_be_bytes().to_vec(),OperationBytes::Update("77".as_bytes().to_vec())),
        (8_i32.to_be_bytes().to_vec(),OperationBytes::Update("88".as_bytes().to_vec())),
        (9_i32.to_be_bytes().to_vec(),OperationBytes::Update("99".as_bytes().to_vec())),
        (10_i32.to_be_bytes().to_vec(),OperationBytes::Update("1010".as_bytes().to_vec())),
    ];

    merkle.insert(prev_key,cur_key.clone(),&mut s,batch.as_slice())?;
    let root = &merkle.root(cur_key,&s)?.unwrap()[..];
    assert_eq!(&[40, 44, 89, 23, 60, 64, 200, 13, 98, 246, 33, 56, 50, 68, 215, 86, 15, 190, 44, 188, 53, 3, 194, 247, 12, 170, 232, 130, 173, 104, 83, 134],root);
    Ok(())
}