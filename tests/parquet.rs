// #![feature(test)]

// extern crate test;

#![allow(clippy::cognitive_complexity, clippy::type_complexity)]

#[cfg(feature = "constellation")]
use constellation::*;
use serde_closure::FnMut;
use std::{
	env, path::PathBuf, time::{Duration, SystemTime}
};
// use test::Bencher;

use amadeus::prelude::*;

fn main() {
	#[cfg(feature = "constellation")]
	init(Resources::default());

	// Accept the number of processes at the command line, defaulting to 10
	let processes = env::args()
		.nth(1)
		.and_then(|arg| arg.parse::<usize>().ok())
		.unwrap_or(10);

	let local_pool_time = {
		let local_pool = LocalPool::new();
		run(&local_pool)
	};
	let thread_pool_time = {
		let thread_pool = ThreadPool::new(processes).unwrap();
		run(&thread_pool)
	};
	#[cfg(feature = "constellation")]
	let process_pool_time = {
		let process_pool = ProcessPool::new(processes, 1, Resources::default()).unwrap();
		run(&process_pool)
	};
	#[cfg(not(feature = "constellation"))]
	let process_pool_time = "-";

	println!(
		"in {:?} {:?} {:?}",
		local_pool_time, thread_pool_time, process_pool_time
	);
}

fn run<P: amadeus_core::pool::ProcessPool>(pool: &P) -> Duration {
	let start = SystemTime::now();

	let rows = Parquet::<_, Value>::new(ParquetDirectory::new(PathBuf::from(
		"amadeus-testing/parquet/cf-accesslogs/",
	)));
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		207_535
	);

	#[cfg(feature = "aws")]
	{
		let rows = Parquet::<_, Value>::new(vec![S3File::new(AwsRegion::UsEast1, "us-east-1.data-analytics", "cflogworkshop/optimized/cf-accesslogs/year=2018/month=11/day=03/part-00137-17868f39-cd99-4b60-bb48-8daf9072122e.c000.snappy.parquet");20]);
		assert_eq!(
			rows.unwrap()
				.dist_iter()
				.map(FnMut!(|row: Result<_, _>| row.unwrap()))
				.count(pool),
			45_167 * 20
		);

		let rows = Parquet::<_, Value>::new(ParquetDirectory::new(S3Directory::new(
			AwsRegion::UsEast1,
			"us-east-1.data-analytics",
			"cflogworkshop/optimized/cf-accesslogs/",
		)));
		assert_eq!(
			rows.unwrap()
				.dist_iter()
				.map(FnMut!(|row: Result<_, _>| row.unwrap()))
				.count(pool),
			207_535
		);

		let rows = Parquet::<_, Value>::new(vec![
			S3File::new(AwsRegion::UsEast1, "us-east-1.data-analytics", "cflogworkshop/optimized/cf-accesslogs/year=2018/month=11/day=02/part-00176-17868f39-cd99-4b60-bb48-8daf9072122e.c000.snappy.parquet"),
			S3File::new(AwsRegion::UsEast1, "us-east-1.data-analytics", "cflogworkshop/optimized/cf-accesslogs/year=2018/month=11/day=02/part-00176-ed461019-4a12-46fa-a3f3-246d58f0ee06.c000.snappy.parquet"),
			S3File::new(AwsRegion::UsEast1, "us-east-1.data-analytics", "cflogworkshop/optimized/cf-accesslogs/year=2018/month=11/day=03/part-00137-17868f39-cd99-4b60-bb48-8daf9072122e.c000.snappy.parquet"),
			S3File::new(AwsRegion::UsEast1, "us-east-1.data-analytics", "cflogworkshop/optimized/cf-accesslogs/year=2018/month=11/day=04/part-00173-17868f39-cd99-4b60-bb48-8daf9072122e.c000.snappy.parquet"),
			S3File::new(AwsRegion::UsEast1, "us-east-1.data-analytics", "cflogworkshop/optimized/cf-accesslogs/year=2018/month=11/day=05/part-00025-17868f39-cd99-4b60-bb48-8daf9072122e.c000.snappy.parquet"),
			S3File::new(AwsRegion::UsEast1, "us-east-1.data-analytics", "cflogworkshop/optimized/cf-accesslogs/year=2018/month=11/day=05/part-00025-96c249f4-3a10-4509-b6b8-693a5d90dbf3.c000.snappy.parquet"),
			S3File::new(AwsRegion::UsEast1, "us-east-1.data-analytics", "cflogworkshop/optimized/cf-accesslogs/year=2018/month=11/day=06/part-00185-96c249f4-3a10-4509-b6b8-693a5d90dbf3.c000.snappy.parquet"),
			S3File::new(AwsRegion::UsEast1, "us-east-1.data-analytics", "cflogworkshop/optimized/cf-accesslogs/year=2018/month=11/day=07/part-00151-96c249f4-3a10-4509-b6b8-693a5d90dbf3.c000.snappy.parquet"),
		]);
		assert_eq!(
			rows.unwrap()
				.dist_iter()
				.map(FnMut!(|row: Result<_, _>| row.unwrap()))
				.count(pool),
			207_535
		);

		let rows = Parquet::<_, Value>::new(ParquetDirectory::new(S3Directory::new(
			AwsRegion::UsEast1,
			"us-east-1.data-analytics",
			"cflogworkshop/optimized/cf-accesslogs/",
		)));
		assert_eq!(
			rows.unwrap()
				.dist_iter()
				.map(FnMut!(|row: Result<_, _>| row.unwrap()))
				.count(pool),
			207_535
		);
	}

	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct StockSimulatedDerived {
		bp1: Option<f64>,
		bp2: Option<f64>,
		bp3: Option<f64>,
		bp4: Option<f64>,
		bp5: Option<f64>,
		bs1: Option<f64>,
		bs2: Option<f64>,
		bs3: Option<f64>,
		bs4: Option<f64>,
		bs5: Option<f64>,
		ap1: Option<f64>,
		ap2: Option<f64>,
		ap3: Option<f64>,
		ap4: Option<f64>,
		ap5: Option<f64>,
		as1: Option<f64>,
		as2: Option<f64>,
		as3: Option<f64>,
		as4: Option<f64>,
		as5: Option<f64>,
		valid: Option<f64>,
		__index_level_0__: Option<i64>,
	}
	let rows = Parquet::<_, StockSimulatedDerived>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/stock_simulated.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		42_000
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/stock_simulated.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: StockSimulatedDerived = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		42_000
	);

	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct StockSimulatedDerivedProjection1 {
		bs5: Option<f64>,
		__index_level_0__: Option<i64>,
	}

	let rows = Parquet::<_, StockSimulatedDerivedProjection1>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/stock_simulated.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		42_000
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/stock_simulated.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: StockSimulatedDerivedProjection1 = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		42_000
	);

	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct StockSimulatedDerivedProjection2 {}

	let rows = Parquet::<_, StockSimulatedDerivedProjection2>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/stock_simulated.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		42_000
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/stock_simulated.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: StockSimulatedDerivedProjection2 = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		42_000
	);

	type TenKayVeeTwo = (
		Vec<u8>,
		i32,
		i64,
		bool,
		f32,
		f64,
		Vec<u8>, // [u8;1024],
		Timestamp,
	);

	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct TenKayVeeTwoDerived {
		binary_field: Vec<u8>,
		int32_field: i32,
		int64_field: i64,
		boolean_field: bool,
		float_field: f32,
		double_field: f64,
		flba_field: Vec<u8>, // [u8;1024],
		int96_field: Timestamp,
	}

	let rows = Parquet::<_, TenKayVeeTwo>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/10k-v2.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		10_000
	);

	let rows = Parquet::<_, TenKayVeeTwoDerived>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/10k-v2.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		10_000
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/10k-v2.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: TenKayVeeTwo = value.clone().downcast().unwrap();
				let _: TenKayVeeTwoDerived = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		10_000
	);

	type AlltypesDictionary = (
		Option<i32>,
		Option<bool>,
		Option<i32>,
		Option<i32>,
		Option<i32>,
		Option<i64>,
		Option<f32>,
		Option<f64>,
		Option<Vec<u8>>,
		Option<Vec<u8>>,
		Option<Timestamp>,
	);

	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct AlltypesDictionaryDerived {
		id: Option<i32>,
		bool_col: Option<bool>,
		tinyint_col: Option<i32>,
		smallint_col: Option<i32>,
		int_col: Option<i32>,
		bigint_col: Option<i64>,
		float_col: Option<f32>,
		double_col: Option<f64>,
		date_string_col: Option<Vec<u8>>,
		string_col: Option<Vec<u8>>,
		timestamp_col: Option<Timestamp>,
	}

	let rows = Parquet::<_, AlltypesDictionary>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/alltypes_dictionary.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		2
	);

	let rows = Parquet::<_, AlltypesDictionaryDerived>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/alltypes_dictionary.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		2
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/alltypes_dictionary.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: AlltypesDictionary = value.clone().downcast().unwrap();
				let _: AlltypesDictionaryDerived = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		2
	);

	type AlltypesPlain = (
		Option<i32>,
		Option<bool>,
		Option<i32>,
		Option<i32>,
		Option<i32>,
		Option<i64>,
		Option<f32>,
		Option<f64>,
		Option<Vec<u8>>,
		Option<Vec<u8>>,
		Option<Timestamp>,
	);

	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct AlltypesPlainDerived {
		id: Option<i32>,
		bool_col: Option<bool>,
		tinyint_col: Option<i32>,
		smallint_col: Option<i32>,
		int_col: Option<i32>,
		bigint_col: Option<i64>,
		float_col: Option<f32>,
		double_col: Option<f64>,
		date_string_col: Option<Vec<u8>>,
		string_col: Option<Vec<u8>>,
		timestamp_col: Option<Timestamp>,
	}

	let rows = Parquet::<_, AlltypesPlain>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/alltypes_plain.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		8
	);

	let rows = Parquet::<_, AlltypesPlainDerived>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/alltypes_plain.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		8
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/alltypes_plain.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: AlltypesPlain = value.clone().downcast().unwrap();
				let _: AlltypesPlainDerived = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		8
	);

	type AlltypesPlainSnappy = (
		Option<i32>,
		Option<bool>,
		Option<i32>,
		Option<i32>,
		Option<i32>,
		Option<i64>,
		Option<f32>,
		Option<f64>,
		Option<Vec<u8>>,
		Option<Vec<u8>>,
		Option<Timestamp>,
	);

	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct AlltypesPlainSnappyDerived {
		id: Option<i32>,
		bool_col: Option<bool>,
		tinyint_col: Option<i32>,
		smallint_col: Option<i32>,
		int_col: Option<i32>,
		bigint_col: Option<i64>,
		float_col: Option<f32>,
		double_col: Option<f64>,
		date_string_col: Option<Vec<u8>>,
		string_col: Option<Vec<u8>>,
		timestamp_col: Option<Timestamp>,
	}

	let rows = Parquet::<_, AlltypesPlainSnappy>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/alltypes_plain.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		2
	);

	let rows = Parquet::<_, AlltypesPlainSnappyDerived>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/alltypes_plain.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		2
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/alltypes_plain.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: AlltypesPlainSnappy = value.clone().downcast().unwrap();
				let _: AlltypesPlainSnappyDerived = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		2
	);

	// TODO

	// type NationDictMalformed = (Option<i32>, Option<Vec<u8>>, Option<i32>, Option<Vec<u8>>);

	// let rows = Parquet::<_,NationDictMalformed>::new(vec![PathBuf::from(
	// 	"amadeus-testing/parquet/nation.dict-malformed.parquet",
	// )]);
	// assert_eq!(
	// 	rows.unwrap().dist_iter().collect::<Vec<_>>(pool),
	// 	[Err(amadeus::source::parquet::Error::Parquet(
	// 		amadeus_parquet::internal::errors::ParquetError::General(String::from(
	// 			"underlying IO error: failed to fill whole buffer"
	// 		))
	// 	))]
	// );

	// let rows = Parquet::<_,Value>::new(vec![PathBuf::from(
	// 	"amadeus-testing/parquet/nation.dict-malformed.parquet",
	// )]);
	// assert_eq!(
	// 	rows.unwrap().dist_iter().collect::<Vec<_>>(pool),
	// 	[Err(amadeus::source::parquet::Error::Parquet(
	// 		amadeus_parquet::internal::errors::ParquetError::General(String::from(
	// 			"underlying IO error: failed to fill whole buffer"
	// 		))
	// 	))]
	// );

	type NestedLists = (
		Option<List<Option<List<Option<List<Option<String>>>>>>>,
		i32,
	);
	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct NestedListsDerived {
		a: Option<List<Option<List<Option<List<Option<String>>>>>>>,
		b: i32,
	}
	let rows = Parquet::<_, NestedLists>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nested_lists.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		3
	);

	let rows = Parquet::<_, NestedListsDerived>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nested_lists.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		3
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nested_lists.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: NestedLists = value.clone().downcast().unwrap();
				let _: NestedListsDerived = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		3
	);

	type NestedMaps = (Option<Map<String, Option<Map<i32, bool>>>>, i32, f64);
	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct NestedMapsDerived {
		a: Option<Map<String, Option<Map<i32, bool>>>>,
		b: i32,
		c: f64,
	}
	let rows = Parquet::<_, NestedMaps>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nested_maps.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		6
	);

	let rows = Parquet::<_, NestedMapsDerived>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nested_maps.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		6
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nested_maps.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: NestedMaps = value.clone().downcast().unwrap();
				let _: NestedMapsDerived = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		6
	);

	type Nonnullable = (
		i64,
		List<i32>,
		List<List<i32>>,
		Map<String, i32>,
		List<Map<String, i32>>,
		(
			i32,
			List<i32>,
			(List<List<(i32, String)>>,),
			Map<String, ((List<f64>,),)>,
		),
	);

	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct NonnullableDerived {
		#[amadeus(name = "ID")]
		id: i64,
		#[amadeus(name = "Int_Array")]
		int_array: List<i32>,
		int_array_array: List<List<i32>>,
		#[amadeus(name = "Int_Map")]
		int_map: Map<String, i32>,
		int_map_array: List<Map<String, i32>>,
		#[amadeus(name = "nested_Struct")]
		nested_struct: NonnullableDerivedInner,
	}

	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct NonnullableDerivedInner {
		a: i32,
		#[amadeus(name = "B")]
		b: List<i32>,
		c: NonnullableDerivedInnerInner,
		#[amadeus(name = "G")]
		g: Map<String, ((List<f64>,),)>,
	}

	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct NonnullableDerivedInnerInner {
		#[amadeus(name = "D")]
		d: List<List<NonnullableDerivedInnerInnerInner>>,
	}

	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct NonnullableDerivedInnerInnerInner {
		e: i32,
		f: String,
	}

	let rows = Parquet::<_, Nonnullable>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nonnullable.impala.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		1
	);

	let rows = Parquet::<_, NonnullableDerived>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nonnullable.impala.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		1
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nonnullable.impala.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: Nonnullable = value.clone().downcast().unwrap();
				let _: NonnullableDerived = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		1
	);

	type Nullable = (
		Option<i64>,
		Option<List<Option<i32>>>,
		Option<List<Option<List<Option<i32>>>>>,
		Option<Map<String, Option<i32>>>,
		Option<List<Option<Map<String, Option<i32>>>>>,
		Option<(
			Option<i32>,
			Option<List<Option<i32>>>,
			Option<(Option<List<Option<List<Option<(Option<i32>, Option<String>)>>>>>,)>,
			Option<Map<String, Option<(Option<(Option<List<Option<f64>>>,)>,)>>>,
		)>,
	);
	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct NullableDerived {
		id: Option<i64>,
		int_array: Option<List<Option<i32>>>,
		#[amadeus(name = "int_array_Array")]
		int_array_array: Option<List<Option<List<Option<i32>>>>>,
		int_map: Option<Map<String, Option<i32>>>,
		#[amadeus(name = "int_Map_Array")]
		int_map_array: Option<List<Option<Map<String, Option<i32>>>>>,
		nested_struct: Option<(
			Option<i32>,
			Option<List<Option<i32>>>,
			Option<(Option<List<Option<List<Option<(Option<i32>, Option<String>)>>>>>,)>,
			Option<Map<String, Option<(Option<(Option<List<Option<f64>>>,)>,)>>>,
		)>,
	}
	let rows = Parquet::<_, Nullable>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nullable.impala.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		7
	);

	let rows = Parquet::<_, NullableDerived>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nullable.impala.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		7
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nullable.impala.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: Nullable = value.clone().downcast().unwrap();
				let _: NullableDerived = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		7
	);

	type Nulls = (Option<(Option<i32>,)>,);
	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct NullsDerived {
		b_struct: Option<(Option<i32>,)>,
	}
	let rows = Parquet::<_, Nulls>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nulls.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		8
	);

	let rows = Parquet::<_, NullsDerived>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nulls.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		8
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/nulls.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: Nulls = value.clone().downcast().unwrap();
				let _: NullsDerived = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		8
	);

	type Repeated = (i32, Option<(List<(i64, Option<String>)>,)>);
	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct RepeatedDerived {
		id: i32,
		#[amadeus(name = "phoneNumbers")]
		phone_numbers: Option<(List<(i64, Option<String>)>,)>,
	}
	let rows = Parquet::<_, Repeated>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/repeated_no_annotation.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		6
	);

	let rows = Parquet::<_, RepeatedDerived>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/repeated_no_annotation.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		6
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/repeated_no_annotation.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: Repeated = value.clone().downcast().unwrap();
				let _: RepeatedDerived = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		6
	);

	type TestDatapage = (Option<String>, i32, f64, bool, Option<List<i32>>);
	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct TestDatapageDerived {
		a: Option<String>,
		b: i32,
		c: f64,
		d: bool,
		e: Option<List<i32>>,
	}
	let rows = Parquet::<_, TestDatapage>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/datapage_v2.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		5
	);

	let rows = Parquet::<_, TestDatapageDerived>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/datapage_v2.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		5
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/datapage_v2.snappy.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: TestDatapage = value.clone().downcast().unwrap();
				let _: TestDatapageDerived = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		5
	);

	#[derive(Data, Clone, PartialEq, PartialOrd, Debug)]
	struct CommitsDerived {
		id: Option<String>,
		delay: Option<i32>,
		age: Option<i32>,
		ismerge: Option<bool>,
		squashof: Option<i32>,
		author_name: Option<String>,
		author_email: Option<String>,
		committer_name: Option<String>,
		committer_email: Option<String>,
		author_time: Option<Timestamp>,
		committer_time: Option<Timestamp>,
		loc_d: Option<i64>,
		loc_i: Option<i64>,
		comp_d: Option<i64>,
		comp_i: Option<i64>,
		nfiles: Option<u16>,
		message: Option<String>,
		ndiffs: Option<u16>,
		author_email_dedup: Option<String>,
		author_name_dedup: Option<String>,
		committer_email_dedup: Option<String>,
		committer_name_dedup: Option<String>,
		__index_level_0__: Option<i64>,
	}

	let rows = Parquet::<_, CommitsDerived>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/commits.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<_, _>| row.unwrap()))
			.count(pool),
		14_444
	);

	let rows = Parquet::<_, Value>::new(vec![PathBuf::from(
		"amadeus-testing/parquet/commits.parquet",
	)]);
	assert_eq!(
		rows.unwrap()
			.dist_iter()
			.map(FnMut!(|row: Result<Value, _>| -> Value {
				let value = row.unwrap();
				let _: CommitsDerived = value.clone().downcast().unwrap();
				value
			}))
			.count(pool),
		14_444
	);

	start.elapsed().unwrap()
}

// #[bench]
// fn record_reader_10k_collect(bench: &mut Bencher) {
// 	let path = Path::new("./amadeus-testing/parquet/10k-v2.parquet");
// 	let file = File::open(&path).unwrap();
// 	let len = file.metadata().unwrap().len();
// 	let parquet_reader = SerializedFileReader::new(file).unwrap();

// 	bench.bytes = len;
// 	bench.iter(|| {
// 		let iter = parquet_reader.get_row_iter(None).unwrap();
// 		println!("{}", iter.count());
// 	})
// }
// #[bench]
// fn record_reader_stock_simulated_collect(bench: &mut Bencher) {
// 	let path = Path::new("./amadeus-testing/parquet/stock_simulated.parquet");
// 	let file = File::open(&path).unwrap();
// 	let len = file.metadata().unwrap().len();
// 	let parquet_reader = SerializedFileReader::new(file).unwrap();

// 	bench.bytes = len;
// 	bench.iter(|| {
// 		let iter = parquet_reader.get_row_iter(None).unwrap();
// 		println!("{}", iter.count());
// 	})
// }

// #[bench]
// fn record_reader_10k_collect_2(bench: &mut Bencher) {
// 	let file = File::open(&Path::new("./amadeus-testing/parquet/10k-v2.parquet")).unwrap();
// 	let len = file.metadata().unwrap().len();
// 	let parquet_reader = SerializedFileReader::new(file).unwrap();

// 	bench.bytes = len;
// 	bench.iter(|| {
// 		let iter =
// 			read2::<_, (Vec<u8>, i32, i64, bool, f32, f64, [u8; 1024], Timestamp)>(&parquet_reader);
// 		println!("{}", iter.unwrap().count());
// 	})
// }
// #[bench]
// fn record_reader_stock_simulated_collect_2(bench: &mut Bencher) {
// 	let path = Path::new("./amadeus-testing/parquet/stock_simulated.parquet");
// 	let file = File::open(&path).unwrap();
// 	let len = file.metadata().unwrap().len();
// 	let parquet_reader = SerializedFileReader::new(file).unwrap();

// 	bench.bytes = len;
// 	bench.iter(|| {
// 		let iter = read2::<
// 			_,
// 			(
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<f64>,
// 				Option<i64>,
// 			),
// 		>(&parquet_reader);
// 		println!("{}", iter.unwrap().count());
// 	})
// }
