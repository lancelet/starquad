#![allow(dead_code)]

extern crate csv;
extern crate flate2;
extern crate serde;
extern crate num;

use csv::{ReaderBuilder, Terminator, Trim};
use flate2::read::GzDecoder;
use serde::Deserialize;
use std::fs::File;
use std::io;
// use std::io::Read;

#[derive(Debug, Deserialize)]
struct GaiaRecord {
    solution_id: u64,
    designation: String,
    source_id: u64,
    random_index: u64,
    ref_epoch: String, // TODO: almost always 2015.5
    ra: f64,
    ra_error: f64,
    dec: f64,
    dec_error: f64,
    parallax: Option<f64>,
    parallax_error: Option<f64>,
    parallax_over_error: Option<f64>,
    pmra: Option<f64>,
    pmra_error: Option<f64>,
    pmdec: Option<f64>,
    pmdec_error: Option<f64>,
    ra_dec_corr: f64,
    ra_parallax_corr: Option<f64>,
    ra_pmra_corr: Option<f64>,
    ra_pmdec_corr: Option<f64>,
    dec_parallax_corr: Option<f64>,
    dec_pmra_corr: Option<f64>,
    dec_pmdec_corr: Option<f64>,
    parallax_pmra_corr: Option<f64>,
    parallax_pmdec_corr: Option<f64>,
    pmra_pmdec_corr: Option<f64>,
    astrometric_n_obs_al: u8,
    astrometric_n_obs_ac: u8,
    astrometric_n_good_obs_al: u8,
    astrometric_n_bad_obs_al: u8,
    astrometric_gof_al: f64,
    astrometric_chi2_al: f64,
    astrometric_excess_noise: f64,
    astrometric_excess_noise_sig: f64,
    astrometric_params_solved: u8,
    astrometric_primary_flag: bool,
    astrometric_weight_al: f64,
    astrometric_pseudo_colour: Option<f64>,
    astrometric_pseudo_colour_error: Option<f64>,
    mean_varpi_factor_al: Option<f64>,
    astrometric_matched_observations: u8,
    visibility_periods_used: u8,
    astrometric_sigma5d_max: f64,
    frame_rotator_object_type: u8,
    matched_observations: u8,
    duplicated_source: bool,
    phot_g_n_obs: u8,
    phot_g_mean_flux: f64,
    phot_g_mean_flux_error: f64,
    phot_g_mean_flux_over_error: f64,
    phot_g_mean_mag: f64,
    phot_bp_n_obs: u8,
    phot_bp_mean_flux: Option<f64>,
    phot_bp_mean_flux_error: Option<f64>,
    phot_bp_mean_flux_over_error: Option<f64>,
    phot_bp_mean_mag: Option<f64>,
    phot_rp_n_obs: u8,
    phot_rp_mean_flux: Option<f64>,
    phot_rp_mean_flux_error: Option<f64>,
    phot_rp_mean_flux_over_error: Option<f64>,
    phot_rp_mean_mag: Option<f64>,
    phot_bp_rp_excess_factor: Option<f64>,
    phot_proc_mode: u8,
    bp_rp: Option<f64>,
    bp_g: Option<f64>,
    g_rp: Option<f64>,
    radial_velocity: Option<f64>,
    radial_velocity_error: Option<f64>,
    rv_nb_transits: u8,
    rv_template_teff: Option<f64>,
    rv_template_logg: Option<f64>,
    rv_template_fe_h: Option<f64>,
    phot_variable_flag: String, // TODO: flag
    l: f64,
    b: f64,
    ecl_lon: f64,
    ecl_lat: f64,
    priam_flags: Option<u64>,
    teff_val: Option<f64>,
    teff_percentile_lower: Option<f64>,
    teff_percentile_upper: Option<f64>,
    a_g_val: Option<f64>,
    a_g_percentile_lower: Option<f64>,
    a_g_percentile_upper: Option<f64>,
    e_bp_min_rp_val: Option<f64>,
    e_bp_min_rp_percentile_lower: Option<f64>,
    e_bp_min_rp_percentile_upper: Option<f64>,
    flame_flags: Option<u64>,
    radius_val: Option<f64>,
    radius_percentile_lower: Option<f64>,
    radius_percentile_upper: Option<f64>,
    lum_val: Option<f64>,
    lum_percentile_lower: Option<f64>,
    lum_percentile_upper: Option<f64>,
}

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let test_file = File::open(
        "/Volumes/Gaia/GaiaSource/GaiaSource_99872562456678912_100222271578464384.csv.gz",
    )?;
    let gz_decoder = GzDecoder::new(&test_file);

    // debugging: dump out each byte from the file to check that gz decoding is working
    /*
    for byte in gz_decoder.bytes() {
        print!("{}", byte.unwrap() as char);
    }
    */

    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .flexible(false)
        .trim(Trim::All)
        .terminator(Terminator::CRLF)
        .quoting(false)
        .from_reader(gz_decoder);

    let records = csv_reader.deserialize::<GaiaRecord>();

    for record in records {
        println!("{:?}", record.unwrap());
    }

    Ok(())
}
