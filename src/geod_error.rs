use crate::calculation::Calculation;
use geographiclib_rs::Geodesic;


// Calculations based on geographiclib@master/tests/GeodTest.cpp

// Math::real angdiff(Math::real a1, Math::real a2) {
fn angdiff(a1: f64, a2: f64) -> f64 {
//   Math::real d = a2 - a1;
    let d = a2 - a1;

    // if (d >= 180)
    //   d -= 360;
    // else if (d < -180)
    //   d += 360;
    // return d;
    if d >= 180.0 {
        d - 360.0
    } else if d < -180.0 {
        d + 360.0
    } else {
        d
    }
}

// Math::real azidiff(Math::real lat,
//                    Math::real lon1, Math::real lon2,
//                    Math::real azi1, Math::real azi2) {
fn azidiff(lat: f64, lon1: f64, lon2: f64, azi1: f64, azi2: f64) -> f64 {
//   Math::real
//     phi = lat * Math::degree(),
    let phi = lat.to_radians();
//     alpha1 = azi1 * Math::degree(),
    let alpha1 = azi1.to_radians();
//     alpha2 = azi2 * Math::degree(),
    let alpha2 = azi2.to_radians();
//     dlam = angdiff(lon1, lon2) * Math::degree();
    let dlam = angdiff(lon1, lon2).to_radians();
//   Math::real res = sin(alpha2-alpha1)*cos(dlam)
//     -cos(alpha2-alpha1)*sin(dlam)*sin(phi)
//     // -sin(alpha1)*cos(alpha2)*(1-cos(dlam))*cos(phi)*cos(phi)
//     ;
//   return res;
    f64::sin(alpha2 - alpha1) * f64::cos(dlam) - f64::cos(alpha2 - alpha1) * f64::sin(dlam) * f64::sin(phi)
}

// Math::real dist(Math::real a, Math::real f,
//                 Math::real lat0, Math::real lon0,
//                 Math::real lat1, Math::real lon1) {
fn distance(a: f64, f: f64, lat0: f64, lon0: f64, lat1: f64, lon1: f64) -> f64 {
    // //  typedef GeographicLib::Math::real real;
    // //  real s12;
    // //  GeographicLib::Geodesic::
    // //    WGS84.Inverse(real(lat0), real(lon0), real(lat1), real(lon1), s12);
    // //  return Math::real(s12);
    // a *= Math::degree();
    let a = a.to_radians();

    // if (abs(lat0 + lat1) > Math::real(179.998)) {
    if (lat0 + lat1).abs() > 179.998 {
        // // Near pole, transform into polar coordinates
        // Math::real
        // r0 = 90 - abs(lat0),
        // r1 = 90 - abs(lat1),
        let r0 = 90.0 - lat0.abs();
        let r1 = 90.0 - lat1.abs();

        // lam0 = lon0 * Math::degree(),
        // lam1 = lon1 * Math::degree();
        let lam0 = lon0.to_radians();
        let lam1 = lon1.to_radians();

        // return (a / (1 - f)) *
        //     Math::hypot
        //         (r0 * cos(lam0) - r1 * cos(lam1), r0 * sin(lam0) - r1 * sin(lam1));
        return (a / (1.0 - f)) * f64::hypot(
            r0 * f64::cos(lam0) - r1 * f64::cos(lam1),
            r0 * f64::sin(lam0) - r1 * f64::sin(lam1),
        );
        // } else {
    } else {
        // // Otherwise use cylindrical formula
        // Math::real
        // phi = lat0 * Math::degree(),
        let phi = lat0.to_radians();
        // cphi = abs(lat0) <= 45 ? cos(phi)
        let cphi = if lat0.abs() <= 45.0 {
            f64::cos(phi)
        } else {
            // : sin((90 - abs(lat0)) * Math::degree()),
            f64::sin((90.0 - lat0.abs()).to_radians())
        };

        // e2 = f * (2 - f),
        let e2 = f * (2.0 - f);
        // sinphi = sin(phi),
        let sinphi = f64::sin(phi);

        // n = 1/sqrt(1 - e2 * sinphi * sinphi),
        let n = 1.0 / f64::sqrt(1.0 - e2 * sinphi * sinphi);

        // // See Wikipedia article on latitude
        // degreeLon = a * cphi * n,
        // degreeLat = a * (1 - e2) * n * n * n,
        let degree_lon = a * cphi * n;
        let degree_lat = a * (1.0 - e2) * n * n * n;

        // dlon = angdiff(lon1, lon0),
        let dlon = angdiff(lon1, lon0);
        // dlat = lat1 - lat0;
        let dlat = lat1 - lat0;
        // dlat *= degreeLat;
        let dlat = dlat * degree_lat;
        // dlon *= degreeLon;
        let dlon = dlon * degree_lon;

        // return Math::hypot(dlat, dlon);
        f64::hypot(dlat, dlon)
    }
}

/*
// err[0] error in position of point 2 for the direct problem.
// err[1] error in azimuth at point 2 for the direct problem.
// err[2] error in m12 for the direct problem & inverse (except near conjugacy)
// err[3] error in s12 for the inverse problem.
// err[4] error in the azimuths for the inverse problem scaled by m12.
// err[5] consistency of the azimuths for the inverse problem.
// err[6] area error direct & inverse (except near conjugacy)
template<class test>
void GeodError(const test& tgeod,
               Math::real lat1, Math::real lon1, Math::real azi1,
               Math::real lat2, Math::real lon2, Math::real azi2,
               Math::real s12, Math::real /*a12*/,
               Math::real m12, Math::real S12,
               vector<Math::real>& err) {
  Math::real tlat1, tlon1, tazi1, tlat2, tlon2, tazi2, ts12, tm12a, tm12b,
    tM12, tM21, tS12a, tS12b /*, ta12*/;
  Math::real rlat1, rlon1, razi1, rlat2, rlon2, razi2, rm12;
  tgeod.Direct(lat1, lon1, azi1,  s12,
               tlat2, tlon2, tazi2, tm12a,
               tM12, tM21, tS12a);
  tS12a -= tgeod.EllipsoidArea() * (tazi2-azi2)/720;
  tgeod.Direct(lat2, lon2, azi2, -s12,
               tlat1, tlon1, tazi1, tm12b,
               tM12, tM21, tS12b);
  tS12b -= tgeod.EllipsoidArea() * (tazi1-azi1)/720;
  err[0] = max(dist(tgeod.EquatorialRadius(), tgeod.Flattening(),
                    lat2, lon2, tlat2, tlon2),
               dist(tgeod.EquatorialRadius(), tgeod.Flattening(),
                    lat1, lon1, tlat1, tlon1));
  err[1] = max(abs(azidiff(lat2, lon2, tlon2, azi2, tazi2)),
               abs(azidiff(lat1, lon1, tlon1, azi1, tazi1))) *
    tgeod.EquatorialRadius();
  err[2] = max(abs(tm12a - m12), abs(tm12b + m12));
  if (!Math::isnan(S12))
    err[6] = max(abs(tS12a - S12), abs(tS12b + S12)) / tgeod.EquatorialRadius();

  /* ta12 = */ tgeod.Inverse(lat1, lon1, lat2, lon2,
                             ts12, tazi1, tazi2, tm12a,
                             tM12, tM21, tS12a);
  tS12a -= tgeod.EllipsoidArea() * ((tazi2-azi2)-(tazi1-azi1))/720;
  err[3] = abs(ts12 - s12);
  err[4] = max(abs(angdiff(azi1, tazi1)), abs(angdiff(azi2, tazi2))) *
    Math::degree() * abs(m12);
  if (lat1 + lat2 == 0)
    err[4] = min(err[4],
                 max(abs(angdiff(azi1, tazi2)), abs(angdiff(azi2, tazi1))) *
                 Math::degree() * abs(m12));
  // m12 and S12 are very sensitive with the inverse problem near conjugacy
  if (!(s12 > tgeod.EquatorialRadius() && m12 < 10e3)) {
    err[2] = max(err[2], abs(tm12a - m12));
    if (!Math::isnan(S12))
      err[6] = max(err[6], abs(tS12a - S12) / tgeod.EquatorialRadius());
  }
  if (s12 > tgeod.EquatorialRadius()) {
    tgeod.Direct(lat1, lon1, tazi1,   ts12/2, rlat2, rlon2, razi2, rm12);
    tgeod.Direct(lat2, lon2, tazi2, - ts12/2, rlat1, rlon1, razi1, rm12);
    err[5] = dist(tgeod.EquatorialRadius(), tgeod.Flattening(),
                  rlat1, rlon1, rlat2, rlon2);
  } else {
    tgeod.Direct(lat1, lon1, tazi1,
                 ts12 + tgeod.EquatorialRadius(),
                 rlat2, rlon2, razi2, rm12);
    tgeod.Direct(lat2, lon2, tazi2, tgeod.EquatorialRadius(),
                 rlat1, rlon1, razi1, rm12);
    err[5] = dist(tgeod.EquatorialRadius(), tgeod.Flattening(),
                  rlat1, rlon1, rlat2, rlon2);
    tgeod.Direct(lat1, lon1, tazi1, - tgeod.EquatorialRadius(),
                 rlat2, rlon2, razi2, rm12);
    tgeod.Direct(lat2, lon2, tazi2,
                 - ts12 - tgeod.EquatorialRadius(),
                 rlat1, rlon1, razi1, rm12);
    err[5] = max(err[5], dist(tgeod.EquatorialRadius(), tgeod.Flattening(),
                              rlat1, rlon1, rlat2, rlon2));
  }
}
*/

#[derive(Debug, Copy, Clone)]
pub struct DirectError {
    pub position_error: f64,
    pub azi_error: f64,
    pub line_number: u32,
    pub calculation: Calculation
}

impl DirectError {
    pub fn new(tgeod: &Geodesic, computed_lat: f64, computed_lon: f64, computed_azi: f64, expected_lat: f64, expected_lon: f64, expected_azi: f64, line_number: u32, calculation: Calculation) -> Self {
        // err[0] = max(dist(tgeod.EquatorialRadius(), tgeod.Flattening(),
        //                   lat2, lon2, tlat2, tlon2),
        //              dist(tgeod.EquatorialRadius(), tgeod.Flattening(),
        //                   lat1, lon1, tlat1, tlon1));
        let position_error =
            distance(tgeod.equatorial_radius(), tgeod.flattening(), expected_lat, expected_lon, computed_lat, computed_lon);

        // err[1] = max(abs(azidiff(lat2, lon2, tlon2, azi2, tazi2)),
        //              abs(azidiff(lat1, lon1, tlon1, azi1, tazi1))) *
        //     tgeod.EquatorialRadius();
        let azi_error = azidiff(expected_lat, expected_lon, computed_lon, expected_azi, computed_azi).abs() * tgeod.equatorial_radius();

        Self { position_error, azi_error, line_number, calculation }
    }
}