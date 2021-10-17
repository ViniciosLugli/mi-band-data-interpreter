//TODO: Set relative type in struct field from mi-band data
mod DataSources {
	use serde::Deserialize;

	#[derive(Debug, Deserialize)]
	pub struct User {
		userId: String,
		gender: String,
		height: String,
		weight: String,
		nickName: String,
		avatar: String,
		birthday: String
	}

	#[derive(Debug, Deserialize)]
	pub struct Activity {
		date: String,
		lastSyncTime: String,
		steps: String,
		distance: String,
		runDistance: String,
		calories: String
	}

	#[derive(Debug, Deserialize)]
	pub struct ActivityMinute {
		date: String,
		time: String,
		steps: String
	}

	#[derive(Debug, Deserialize)]
	pub struct ActivityStage {
		date: String,
		start: String,
		stop: String,
		distance: String,
		calories: String,
		steps: String
	}

	#[derive(Debug, Deserialize)]
	pub struct Body {
		timestamp: String,
		weight: String,
		height: String,
		bmi: String,
		fatRate: String,
		bodyWaterRate: String,
		boneMass: String,
		metabolism: String,
		muscleRate: String,
		visceralFat: String,
		impedance: String
	}

	#[derive(Debug, Deserialize)]
	pub struct Heartrate {
		date: String,
		lastSyncTime: String,
		heartRate: String,
		timestamp: String
	}

	#[derive(Debug, Deserialize)]
	pub struct HeartrateAuto {
		date: String,
		time: String,
		heartRate: String
	}

	#[derive(Debug, Deserialize)]
	pub struct Sleep {
		date: String,
		lastSyncTime: String,
		deepSleepTime: String,
		shallowSleepTime: String,
		wakeTime: String,
		start: String,
		stop: String
	}

	#[derive(Debug, Deserialize)]
	pub struct Sport {
		r#type: String,
		startTime: String,
		sportTime: String,
		distance: String,
		maxPace: String,
		minPace: String,
		avgPace: String,
		calories: String
	}
}

mod Importer {
	use csv;

	#[macro_export]
	macro_rules! read_from_file {
		($path: expr,$typ: ty) => {
			let mut rdr = csv::Reader::from_path($path.to_string()).unwrap();

			for result in rdr.deserialize() {
				let record: $typ = result.unwrap();
				println!("{:?}", record);
			}
		};
	}
}

mod Formatter {}

mod Processors {}

fn main() {}
