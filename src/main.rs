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

	#[derive(Debug, Deserialize)]
	pub struct All {
		pub user: User,
		pub activity: Activity,
		pub activity_minute: ActivityMinute,
		pub activity_stage: ActivityStage,
		pub body: Body,
		pub heartrate: Heartrate,
		pub heartrate_auto: HeartrateAuto,
		pub sleep: Sleep,
		pub sport: Sport
	}
}

mod Importer {
	#[macro_export]
	macro_rules! read_from_file {
		($path: expr,$typ: ty) => {{
			let filename = glob::glob(&format!("{}/*", $path)).unwrap().next().unwrap().unwrap();
			let path = format!("./{}", filename.into_os_string().into_string().unwrap());

			let mut rdr = csv::Reader::from_path(path).unwrap();

			let record: $typ = rdr.deserialize().next().unwrap().unwrap();
			println!("New record: {:?}", record);
			record
		}};
	}
}

//mod Formatter {}

mod Processor {
	#![macro_use]

	pub fn export(base_path: &'static str) -> crate::DataSources::All {
		let response_data: crate::DataSources::All = crate::DataSources::All {
			user: crate::read_from_file!(format!("{}/USER", base_path), crate::DataSources::User),
			activity: crate::read_from_file!(format!("{}/ACTIVITY", base_path), crate::DataSources::Activity),
			activity_minute: crate::read_from_file!(format!("{}/ACTIVITY_MINUTE", base_path), crate::DataSources::ActivityMinute),
			activity_stage: crate::read_from_file!(format!("{}/ACTIVITY_STAGE", base_path), crate::DataSources::ActivityStage),
			body: crate::read_from_file!(format!("{}/BODY", base_path), crate::DataSources::Body),
			heartrate: crate::read_from_file!(format!("{}/HEARTRATE", base_path), crate::DataSources::Heartrate),
			heartrate_auto: crate::read_from_file!(format!("{}/HEARTRATE_AUTO", base_path), crate::DataSources::HeartrateAuto),
			sleep: crate::read_from_file!(format!("{}/SLEEP", base_path), crate::DataSources::Sleep),
			sport: crate::read_from_file!(format!("{}/SPORT", base_path), crate::DataSources::Sport)
		};
		response_data
	}
}

fn main() {
	println!("{:?}", Processor::export("./target/ViniciosData"));
}
