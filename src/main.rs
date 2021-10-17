//TODO: Set relative type in struct field from mi-band data
mod DataSources {
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Serialize, Deserialize)]
	pub struct User {
		userId: String,
		gender: String,
		height: String,
		weight: String,
		nickName: String,
		avatar: String,
		birthday: String
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Activity {
		date: String,
		lastSyncTime: String,
		steps: String,
		distance: String,
		runDistance: String,
		calories: String
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct ActivityMinute {
		date: String,
		time: String,
		steps: String
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct ActivityStage {
		date: String,
		start: String,
		stop: String,
		distance: String,
		calories: String,
		steps: String
	}

	#[derive(Debug, Serialize, Deserialize)]
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

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Heartrate {
		date: String,
		lastSyncTime: String,
		heartRate: String,
		timestamp: String
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct HeartrateAuto {
		date: String,
		time: String,
		heartRate: String
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Sleep {
		date: String,
		lastSyncTime: String,
		deepSleepTime: String,
		shallowSleepTime: String,
		wakeTime: String,
		start: String,
		stop: String
	}

	#[derive(Debug, Serialize, Deserialize)]
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

	#[derive(Debug, Serialize, Deserialize)]
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
			let mut record: $typ = rdr.deserialize().next().unwrap().unwrap();
			for result in rdr.deserialize() {
				record = result.unwrap();
				println!("{:?}", record);
			}
			println!("New record: {:?}", record);
			record
		}};
	}
}

//mod Formatter {}

mod Processor {
	#![macro_use]

	pub fn export(base_path: String) -> crate::DataSources::All {
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

mod Api {

	use std::path::PathBuf;

	use actix_multipart::Multipart;
	use actix_web::{post, web, App, Error, HttpResponse, HttpServer};
	use async_std::prelude::*;
	use futures_util::TryStreamExt as _;
	use uuid::Uuid;
	use zip_extensions::*;

	pub async fn format_user_data(mut payload: Multipart) -> Result<HttpResponse, Error> {
		let mut filepath = "./".to_string();
		let mut outpath = "./tmp/data/";

		// iterate over multipart stream
		while let Ok(Some(mut field)) = payload.try_next().await {
			let content_disposition = field.content_disposition().ok_or_else(|| HttpResponse::BadRequest().finish())?;

			let filename = content_disposition.get_filename().map_or_else(|| Uuid::new_v4().to_string(), |f| sanitize_filename::sanitize(f));
			filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));

			let mut f = async_std::fs::File::create(&filepath).await?;

			// Field in turn is stream of *Bytes* object
			while let Some(chunk) = field.try_next().await? {
				f.write_all(&chunk).await?;
			}
		}

		let archive_file: PathBuf = PathBuf::from(&filepath);
		let target_dir: PathBuf = PathBuf::from(&outpath);
		zip_extract(&archive_file, &target_dir).unwrap();

		let output_info: crate::DataSources::All = crate::Processor::export(format!("{}stats", &outpath));

		async_std::fs::remove_file(&filepath);
		async_std::fs::remove_file(&outpath);

		// FAKE DATA FOR TESTS AND HACKATON
		let html = r#"<!DOCTYPE html>
		<html lang='pt-br'>

		<head>
			<meta charset='utf-8'>
			<meta name='viewport' content='width=device-width, initial-scale=1.0'>
			<meta http-equiv='X-UA-Compatible' content='IE=edge'>
			<script src='https://cdnjs.cloudflare.com/ajax/libs/Chart.js/3.5.1/chart.min.js'></script>
			<link rel='stylesheet' href='https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css'>
			<script src='https://cdnjs.cloudflare.com/ajax/libs/patternomaly/1.3.2/patternomaly.js' integrity='sha512-gNM40ajr/bSi3Af8i6D4dV2CUWZrkm2zhgeWf46H91zOwWoH8Wwsyf6kQ4syfNyOrnjATrjKkP4ybWD7eKp2KA==' crossorigin='anonymous' referrerpolicy='no-referrer'></script>
			<title>Paciente: Vinicios Lugli</title>
		</head>

		<body>
			<div class='container' style='position: absolute; left: 400px;width:480px; height:480px;'>
				<canvas id='pie'></canvas>
			</div>
			<div class='container' style='position: absolute; right: 400px;padding-bottom:50px;width:480px; height:480px;'>
				<canvas id='radar'></canvas>
			</div>
			<div class='container' style='position: absolute; top: 440px;left: 500px;width:1000px; padding-bottom:50px;'>
				<canvas id='graph'></canvas>
			</div>
			<script>
				let graph = document.getElementById('graph').getContext('2d');
				let pie = document.getElementById('pie').getContext('2d');
				let radar = document.getElementById('radar').getContext('2d');

				function randomIntFromInterval(min, max) {
					return Math.floor(Math.random() * (max - min + 1) + min)
				}

				function fillTestArray(len, min, max) {
					var arr = [];
					for (var i = 0; i < len; i++) {
						arr.push(randomIntFromInterval(min, max));
					}
					return arr;
				}

				let graph_data = new Chart(graph, {
					type: 'line',
					data: {
						labels: ['segunda', 'terça', 'quarta', 'quinta', 'sexta'],
						datasets: [{
							label: 'passos',
							data: fillTestArray(5, 256, 1024),
							backgroundColor: 'rgba(54, 162, 235, 0.6)',
							borderWidth: 5,
							borderColor: pattern.draw('circle', '#36a2eb'),
							hoverBorderWidth: 2,
							hoverBorderColor: '#000'
						},
						{
							label: 'Batimentos',
							data: fillTestArray(5, 60, 110).reverse(),
							backgroundColor: 'rgba(255, 99, 132, 0.6)',
							borderWidth: 5,
							borderColor: '#ff9fb4',
							hoverBorderWidth: 2,
							hoverBorderColor: '#000'
						},
						{
							label: 'Calorias',
							data: fillTestArray(5, 720, 1200).reverse(),
							backgroundColor: 'rgba(255, 206, 86, 0.6)',
							borderWidth: 5,
							borderColor: pattern.draw('ring', '#FFEA79'),
							hoverBorderWidth: 2,
							hoverBorderColor: '#000'
						}]
					},
					options: {
						plugins: {
							title: {
								display: true,
								text: 'Monitoramento',
								font: {
									size: 25
								}
							},
						}
					}
				})

				let pie_data = new Chart(pie, {
					type: 'pie',
					data: {
						labels: ['Ociosidade', 'Corrida', 'Futebol', 'Basquete', 'Descanso'],
						datasets: [{
							label: 'Pesos de atividades executadas',
							data: [randomIntFromInterval(12, 23), randomIntFromInterval(6, 20), randomIntFromInterval(4, 8), randomIntFromInterval(3, 6), randomIntFromInterval(5, 8)],
							borderWidth: 1,
							backgroundColor: [
							pattern.draw('zigzag', '#888'),
							pattern.draw('square', '#F1C40F'),
							pattern.draw('ring', '#27AE60'),
							pattern.draw('diamond', '#D35400'),
							pattern.draw('diagonal', '#1F618D')]
						}]
						},
					options: {
					}
				})

				let radar_data = new Chart(radar, {
					type: 'radar',
					data: {
						labels: ['segunda', 'terça', 'quarta', 'quinta', 'sexta'],
						datasets: [{
							label: 'Sono profundo',
							data:  [100, 100, 100, 100, 100],
							borderWidth: 1,
							borderColor: 'black',
							backgroundColor: '#1F618D',
						}
						]
						},
					options: {
						title: {
								display: true,
								text: 'Informações do paciente',
								font: {
									size: 25
							},
						}
					}
				})

			</script>

		</body>

		</html>"#;

		Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
	}

	pub fn index() -> HttpResponse {
		let html = r#"<html>
		<head><title>Upload Test</title></head>
		<style>
			body{
				background: #222222;
			}
			form{
				position: absolute;
				top: 50%;
				left: 50%;
				margin-top: -100px;
				margin-left: -250px;
				width: 500px;
				height: 200px;
				border: 4px dashed #fff;
			}
			form p{
				width: 100%;
				height: 100%;
				text-align: center;
				line-height: 170px;
				color: #ffffff;
				font-family: Arial;
			}
			form input{
				position: absolute;
				margin: 0;
				padding: 0;
				width: 100%;
				height: 100%;
				outline: none;
				opacity: 0;
			}
			form button{
				margin: 0;
				color: #fff;
				background: #FF6188;
				border: none;
				width: 508px;
				height: 35px;
				margin-top: -20px;
				margin-left: -4px;
				border-radius: 4px;
				border-bottom: 4px solid #ff7a9c;
				transition: all .2s ease;
				outline: none;
			}
			form button:hover{
				background: #ff668c;
				color: #ff4d79;
			}
			form button:active{
				border:0;
			}
		</style>
		<body>
			<form target="/" method="post" enctype="multipart/form-data">
				<input type="file" multiple name="file"/>
				<p>Arraste seu arquivo ou clique para seleciona-lo!.<br />
					Arquivo de informações pessoais de seu pulseira.
				</p>
				<button type="submit">Submit</button>
			</form>
		</body>
		<script>
			$(document).ready(function(){
				$('form input').change(function () {
					$('form p').text(this.files.length + " arquivo selecionado");
				});
			});
		</script>
	</html>
	"#;

		HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
	}
}

use actix_web::{middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "info");
	std::fs::create_dir_all("./tmp")?;
	println!("gaass");
	HttpServer::new(|| {
		App::new()
			.wrap(middleware::Logger::default())
			.service(web::resource("/").route(web::get().to(Api::index)).route(web::post().to(Api::format_user_data)))
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}

