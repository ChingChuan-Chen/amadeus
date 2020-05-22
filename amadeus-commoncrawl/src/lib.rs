#![doc(html_root_url = "https://docs.rs/amadeus-commoncrawl/0.1.7")]
#![feature(type_alias_impl_trait)]

mod commoncrawl;
mod parser;

use async_compression::futures::bufread::GzipDecoder; // TODO: use stream or https://github.com/alexcrichton/flate2-rs/pull/214
use futures::{io::BufReader, AsyncBufReadExt, FutureExt, StreamExt, TryStreamExt};
use reqwest_resume::ClientExt;
use serde_closure::*;
use std::{io, iter, time};

use amadeus_core::{
	dist_iter::DistributedIterator, into_dist_iter::IntoDistributedIterator, Source
};
use amadeus_types::Webpage;

use commoncrawl::WarcParser;

/// See https://commoncrawl.s3.amazonaws.com/crawl-data/index.html
/// CC-MAIN-2018-43
pub struct CommonCrawl {
	urls: Vec<String>,
}
impl CommonCrawl {
	pub async fn new(id: &str) -> Result<Self, reqwest::Error> {
		let url = format!(
			"https://commoncrawl.s3.amazonaws.com/crawl-data/{}/warc.paths.gz",
			id
		);
		let body = reqwest::ClientBuilder::new()
			.timeout(time::Duration::new(120, 0))
			.build()
			.unwrap()
			.resumable()
			.get(url.parse().unwrap())
			.send();
		let body = body
			.await?
			.bytes_stream()
			.map_err(|e| io::Error::new(io::ErrorKind::Other, e));
		let body = BufReader::new(body.into_async_read());
		let mut body = GzipDecoder::new(body); // Content-Encoding isn't set, so decode manually
		body.multiple_members(true);

		let urls = BufReader::new(body)
			.lines()
			.map(FnMut!(|url: Result<String, io::Error>| -> String {
				format!("http://commoncrawl.s3.amazonaws.com/{}", url.unwrap())
			}))
			.collect()
			.await;
		Ok(Self { urls })
	}
}

// let body = reqwest::get(
// 	"http://commoncrawl.s3.amazonaws.com/crawl-data/CC-MAIN-2018-30/warc.paths.gz",
// )
// .await
// .unwrap();
// let body = body
// 	.bytes_stream()
// 	.map_err(|e| io::Error::new(io::ErrorKind::Other, e));
// let body = BufReader::new(body.into_async_read());
// let mut body = GzipDecoder::new(body); // Content-Encoding isn't set, so decode manually
// body.multiple_members(true);
// let handles = BufReader::new(body)
// 	.lines()
// 	.map(|url| format!("http://commoncrawl.s3.amazonaws.com/{}", url.unwrap()))
// 	.take(10)
// 	.map(|url| {
// 		tokio::spawn(async move {
// 			println!("{}", url);
// 			let body = super::get(url.parse().unwrap()).await.unwrap();
// 			let body = body
// 				.bytes_stream()
// 				.map_err(|e| io::Error::new(io::ErrorKind::Other, e));
// 			let body = BufReader::new(body.into_async_read());
// 			let mut body = GzipDecoder::new(body); // Content-Encoding isn't set, so decode manually
// 			body.multiple_members(true);
// 			let n = futures::io::copy(&mut body, &mut futures::io::sink())
// 				.await
// 				.unwrap();
// 			println!("{}", n);
// 		})
// 	})
// 	.collect::<Vec<_>>()
// 	.await;

impl Source for CommonCrawl {
	type Item = Webpage<'static>;
	type Error = io::Error;

	#[cfg(not(feature = "doc"))]
	type DistIter = impl DistributedIterator<Item = Result<Self::Item, Self::Error>>;
	#[cfg(feature = "doc")]
	type DistIter = amadeus_core::util::ImplDistributedIterator<Result<Self::Item, Self::Error>>;
	type Iter = iter::Empty<Result<Self::Item, Self::Error>>;

	#[allow(clippy::let_and_return)]
	fn dist_iter(self) -> Self::DistIter {
		let ret = self
			.urls
			.into_dist_iter()
			.flat_map(FnMut!(|url: String| async move {
				let body = reqwest_resume::get(url.parse().unwrap()).await.unwrap();
				let body = body
					.bytes_stream()
					.map_err(|e| io::Error::new(io::ErrorKind::Other, e));
				let body = BufReader::new(body.into_async_read());
				let mut body = GzipDecoder::new(body); // Content-Encoding isn't set, so decode manually
				body.multiple_members(true);
				WarcParser::new(body)
			}
			.flatten_stream()));
		#[cfg(feature = "doc")]
		let ret = amadeus_core::util::ImplDistributedIterator::new(ret);
		ret
	}
	fn iter(self) -> Self::Iter {
		iter::empty()
	}
}
