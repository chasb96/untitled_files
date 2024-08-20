use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;

#[derive(Copy, Clone, PartialEq)]
pub enum FileFormat {
    StereolithographyBinary,
    StandardForTheExchangeOfProductModelData,
    JointPhotographicExpertsGroup,
    PlainText,
    Markdown,
    Blender,
    PortableNetworkGraphics,
}

impl FileFormat {
    pub async fn from_read_magic_bytes<R>(reader: &mut R) -> Result<Option<Self>, std::io::Error> 
    where
        R: AsyncRead + Unpin
    {
        let mut buffer = [0u8; 16];

        reader
            .read_exact(&mut buffer)
            .await?;

        for magic_bytes in MAGIC_BYTES {
            let signature = &buffer[magic_bytes.offset..magic_bytes.bytes.len()];

            if signature.eq(magic_bytes.bytes) {
                return Ok(Some(magic_bytes.format));
            }
        }

        Ok(None)
    }

    pub async fn from_extension(extension: &str) -> Option<Self> {
        match extension.to_lowercase().as_str() {
            "stl" => Some(Self::StereolithographyBinary),
            "step" => Some(Self::StandardForTheExchangeOfProductModelData),
            "jpg" => Some(Self::JointPhotographicExpertsGroup),
            "jpeg" => Some(Self::JointPhotographicExpertsGroup),
            "txt" => Some(Self::PlainText),
            "md" => Some(Self::Markdown),
            "blend" => Some(Self::Blender),
            "png" => Some(Self::PortableNetworkGraphics),
            _ => None,
        }
    }

    pub fn media_type(&self) -> &'static str {
        match self {
            Self::StereolithographyBinary => "model/x.stl-binary",
            Self::StandardForTheExchangeOfProductModelData => "model/step",
            Self::JointPhotographicExpertsGroup => "image/jpeg",
            Self::PlainText => "text/plain",
            Self::Markdown => "text/markdown",
            Self::Blender => "application/x-blender",
            Self::PortableNetworkGraphics => "image/png",
        }
    }
}

struct MagicBytes {
    bytes: &'static [u8],
    offset: usize,
    format: FileFormat,
}

const MAGIC_BYTES: &'static [MagicBytes] = &[
    MagicBytes { bytes: b"STLB ATF ", offset: 0, format: FileFormat::StereolithographyBinary },
    MagicBytes { bytes: b"ISO-10303-21;", offset: 0, format: FileFormat::StandardForTheExchangeOfProductModelData },
    MagicBytes { bytes: b"\xff\xd8\xff\xdb", offset: 0, format: FileFormat::JointPhotographicExpertsGroup },
    MagicBytes { bytes: b"\xff\xd8\xff\xee", offset: 0, format: FileFormat::JointPhotographicExpertsGroup },
    MagicBytes { bytes: b"\xff\xd8\xff\xe1", offset: 0, format: FileFormat::JointPhotographicExpertsGroup },
    MagicBytes { bytes: b"\xff\xd8\xff\xe0", offset: 0, format: FileFormat::JointPhotographicExpertsGroup },
    MagicBytes { bytes: b"BLENDER", offset: 0, format: FileFormat::Blender },
    MagicBytes { bytes: b"\x89PNG\r\n\x1A\n", offset: 1, format: FileFormat::PortableNetworkGraphics },
];