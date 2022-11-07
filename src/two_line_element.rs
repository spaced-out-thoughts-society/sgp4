/* Two-line-element stallite orbital data */
pub struct TwoLineElement {
    float: epoch,              /* Epoch Time in NORAD TLE format YYDDD.FFFFFFFF */
    u32: epoch_year,           /* Epoch: year */
    u32: epoch_day,            /* Epoch: day of year */
    float: epoch_fod,          /* Epoch: fraction of day */
    float: xndt20,             /* 1. time derivate of mean motion */
    float: xndd6o,             /* 2. time derivative of mean motion */
    float: bstar,              /* Bstar drag coefficient */
    float: xincl,              /* Inclination */
    float: xnodeo,             /* R.A.A.N. */
    float: eo,                 /*Eccentricity */
    float: omegao,             /* argument of perigee */
    float: xmo,                /* mean anomaly */
    float: xno,                /* mean motion */
    i32: catnr,                /*Catalogue number */
    i32: elset,                /* Element Set Number */
    i32: revnum,               /* Recolution Number at epoch */
    char: sat_name,            /* Satellite name string */
    char: idseg,               /*Iternational Designator */
    OperationalStatus: status, /* Operational Status */

    /* values needed for squint calculations */
    float: xincl1,
    float: xnodeo1,
    float: omega01,
}
