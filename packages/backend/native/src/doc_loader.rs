use affine_common::{doc_loader::Doc, napi_utils::map_napi_err};
use napi::{Result, Status, bindgen_prelude::Buffer};
use napi_derive::napi;

#[napi(object)]
pub struct Chunk {
  pub index: i64,
  pub content: String,
}

#[napi(object)]
pub struct ParsedDoc {
  pub name: String,
  pub chunks: Vec<Chunk>,
}

#[napi]
pub fn parse_doc(file_path: String, doc: Buffer) -> Result<ParsedDoc> {
  let inner = map_napi_err(Doc::new(&file_path, &doc), Status::GenericFailure)?;
  let chunks = inner
    .chunks
    .iter()
    .enumerate()
    .map(|(i, chunk)| {
      let content = crate::utils::clean_content(&chunk.content);
      Chunk {
        index: i as i64,
        content,
      }
    })
    .collect::<Vec<Chunk>>();
  Ok(ParsedDoc {
    name: inner.name.clone(),
    chunks,
  })
}
