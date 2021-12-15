use bs3::backend::MemoryBackend;
use bs3::merkle::{append_only, Merkle};
use bs3::OperationBytes;
use bs3::Result;
use sha3::Sha3_256;

#[test]
fn test() -> Result<()> {
    let mut s = MemoryBackend::new();
    let mut merkle = append_only::AppendOnlyMerkle::<Sha3_256>::new("merkle_test", 0);
    // at height 0 an empty array will be inserted
    merkle.insert(&mut s, vec![].as_slice())?;

    let batch = vec![
        (
            1_i32.to_be_bytes().to_vec(),
            OperationBytes::Update("11".as_bytes().to_vec()),
        ),
        (
            2_i32.to_be_bytes().to_vec(),
            OperationBytes::Update("22".as_bytes().to_vec()),
        ),
        (
            3_i32.to_be_bytes().to_vec(),
            OperationBytes::Update("33".as_bytes().to_vec()),
        ),
        (
            4_i32.to_be_bytes().to_vec(),
            OperationBytes::Update("44".as_bytes().to_vec()),
        ),
        (
            5_i32.to_be_bytes().to_vec(),
            OperationBytes::Update("55".as_bytes().to_vec()),
        ),
    ];

    merkle.insert(&mut s, batch.as_slice())?;
    let root = &merkle.root(&s)?[..];
    assert_eq!(
        &[
            46, 134, 9, 103, 111, 12, 217, 43, 135, 206, 43, 136, 243, 134, 104, 67, 60, 116, 45,
            163, 158, 138, 169, 110, 247, 185, 241, 132, 219, 10, 187, 203
        ],
        root
    );

    let batch = vec![
        (
            6_i32.to_be_bytes().to_vec(),
            OperationBytes::Update("66".as_bytes().to_vec()),
        ),
        (
            7_i32.to_be_bytes().to_vec(),
            OperationBytes::Update("77".as_bytes().to_vec()),
        ),
        (
            8_i32.to_be_bytes().to_vec(),
            OperationBytes::Update("88".as_bytes().to_vec()),
        ),
        (
            9_i32.to_be_bytes().to_vec(),
            OperationBytes::Update("99".as_bytes().to_vec()),
        ),
        (
            10_i32.to_be_bytes().to_vec(),
            OperationBytes::Update("1010".as_bytes().to_vec()),
        ),
    ];

    merkle.insert(&mut s, batch.as_slice())?;
    let root = &merkle.root(&s)?[..];
    assert_eq!(
        &[
            98, 207, 88, 202, 196, 54, 150, 47, 166, 102, 21, 245, 140, 111, 84, 112, 164, 213,
            135, 164, 2, 169, 72, 222, 79, 16, 162, 0, 65, 230, 197, 69
        ],
        root
    );
    Ok(())
}
