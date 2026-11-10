#[derive(Debug, PartialEq, Eq)]
enum ImageFormat {
    Png,
    Jpeg,
    Gif,
}

fn detect_format(_bytes: &[u8]) -> Option<ImageFormat> {
    todo!()
}

fn main() {
    let png = b"\x89PNG\r\n\x1a\n";
    let jpeg = b"\xff\xd8\xff";
    let gif87 = b"GIF87a";
    let gif89 = b"GIF89a";

    assert_eq!(detect_format(png), Some(ImageFormat::Png));
    assert_eq!(detect_format(jpeg), Some(ImageFormat::Jpeg));
    assert_eq!(detect_format(gif87), Some(ImageFormat::Gif));
    assert_eq!(detect_format(gif89), Some(ImageFormat::Gif));
    assert_eq!(
        detect_format(b"\x89PNG\r\n\x1a\nextra"),
        Some(ImageFormat::Png)
    );
    assert_eq!(
        detect_format(b"\xff\xd8\xff\xe0\x00\x10"),
        Some(ImageFormat::Jpeg)
    );
    assert_eq!(
        detect_format(b"GIF89a\x01\x00\x01\x00"),
        Some(ImageFormat::Gif)
    );

    for signature in [&png[..], &jpeg[..], &gif87[..], &gif89[..]] {
        for length in 0..signature.len() {
            assert_eq!(detect_format(&signature[..length]), None);
        }
        for index in 0..signature.len() {
            let mut damaged = signature.to_vec();
            damaged[index] = 0;
            assert_eq!(detect_format(&damaged), None);
        }
    }

    assert_eq!(detect_format(b"GIF88a"), None);
    assert_eq!(detect_format(b"GIF89A"), None);
    assert_eq!(detect_format(b"gif89a"), None);
    assert_eq!(detect_format(b"PNG"), None);
    assert_eq!(detect_format(b"\x00GIF89a"), None);
    assert_eq!(detect_format(b"not an image"), None);

    println!("formats: ok");
}
