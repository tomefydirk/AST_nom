pub fn my_abs(nb: f64) -> f64 {
    if nb < 0.0 {
        -nb
    } else {
        nb
    }
}
pub fn my_sqrt(nb: f64) -> f64 {
    if nb < 0.0 {
        panic!("error sqrt ne supporte pas les nombres inférieurs à 0");
    }
    let mut xn = nb / 2.0;
    let epsilon = 0.00000000001;
    loop {
        xn = (xn + (nb / xn)) / 2.0;
        let c = (xn * xn) - nb;
        if my_abs(c) < epsilon {
            break;
        }
    }
    xn
}
pub fn my_ln(nb: f64) -> f64 {

    if nb <= 0.0 {
        panic!("error ln ne supporte les nombres inférrieurs à 0(ou égale)");
    }

    let mut a = nb;
    let mut count = 0;
    
    let epsilon = 0.0000000001;
    while my_abs(a - 1.0) > epsilon {
        a = my_sqrt(a);
        count += 1;
    }
    {}
    a -= 1.0000;
    while count > 0 {
        a *= 2.00;
        count -= 1;
    }
    a
}
