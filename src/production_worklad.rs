use libm::{cos, exp, fabs, log, sin, sqrt}; 

type WhetFloat = f64; 

pub fn small_whetstone(kilo_whets: usize) {
    const T: WhetFloat = 0.499975;
    const T1: WhetFloat = 0.50025;
    const T2: WhetFloat = 2.0;

    const N8: i32 = 10;
    const N9: i32 = 5;

    const VALUE: WhetFloat = 0.941377;
    const TOLERANCE: WhetFloat = 0.00001;

    const Y: WhetFloat = 1.0;

    fn clear_array(e1: &mut [WhetFloat; (N9 + 1) as usize]) {
        for value in e1.iter_mut() {
            *value = 0.0;
        }
    }

    fn p3(x: WhetFloat, y: WhetFloat, z: &mut WhetFloat) {
        let x_temp = T * (x + *z);
        let y_temp = T * (x_temp + y);
        *z = (x_temp + y_temp) / T2;
    }

    let mut _sum = 0.0;
    let mut e1: [WhetFloat; (N9 + 1) as usize] = [0.0; (N9 + 1) as usize];
    let mut ij: i32 = 1;
    let mut ik = 2;
    let mut il = 3;
    
    for _ in 0..kilo_whets {
        clear_array(&mut e1);

        // Module 6: Integer arithmetic
        ij = (ik - ij) * (il - ik);
        ik = il - (ik - ij);
        il = (il - ik) * (ik + il);

        if ik < 1 || il < 1 {
            return;
        } else if ik - 1 > N9 || il - 1 > N9 {
            return;
        } else {
            e1[(il - 1) as usize] = (ij + il + ik) as WhetFloat;
            e1[(ik - 1) as usize] = sin(il as WhetFloat);
        }

        // Module 8: Procedure calls
        let mut z = e1[3];
        for inner_loop_var in 1..=N8 {
            p3(Y * inner_loop_var as WhetFloat, Y + z, &mut z);
        }

        // Second version of Module 6
        ij = il - (il - 3) * ik;
        il = (il - ik) * (ik - ij);
        ik = (il - ik) * ik;

        if (il - 1) < 1 {
            return;
        } else if (il - 1) > N9 {
            return;
        } else {
            e1[(il - 1) as usize] = (ij + il + ik) as WhetFloat;
        }

        if (ik + 1) > N9 {
            return;
        } else if (ik + 1) < 1 {
            return;
        } else {
            e1[(ik + 1) as usize] = fabs(cos(z));
        }

        // Module 11: Standard mathematical functions
        z = sqrt(exp(log(e1[(N9 - 1) as usize]) / T1));
        _sum += z;

        
        if fabs(z - VALUE) > TOLERANCE {
            _sum *= 2.0; // Forces error at end
            //ij += 1; // Prevents optimization CAUSES EXCEPTION!!
        }
    }
}


/*
pub fn small_whetstone1(kilo_whets: usize) {
    const T: WhetFloat = 0.499975;
    const T1: WhetFloat = 0.50025;
    const T2: WhetFloat = 2.0;

    const VALUE: WhetFloat = 0.941377;
    const TOLERANCE: WhetFloat = 0.00001;

    const ARRAY_SIZE: usize = 5;

    let mut _sum: WhetFloat = 0.0;
    let mut e1: [WhetFloat; ARRAY_SIZE] = [0.0; 5];
    let mut j: i32 = 1;
    let mut k = 2;
    let mut l = 3;

    fn clear_array(e1: &mut [WhetFloat; ARRAY_SIZE]) {
        for value in e1.iter_mut() {
            *value = 0.0;
        }
    }
    fn p3(x: WhetFloat, y: WhetFloat, z: &mut WhetFloat) {
        let mut x1 = x;
        let mut y1 = y;
        x1 = T * (x1 + y1);
        y1 = T * (x1 + y1);
        *z = (x1 + y1) / T2;
    }

    // for _ in 1..=kilo_whets {
    //     j = j * (k - j) * (l - k);
    //     k = l * k - (l - j) * k;
    //     l = (l - k) * (k + j);
    //     e1[(l - 1) as usize] = (j + k + l) as WhetFloat;
    //     e1[(k - 1) as usize] = (j * k * l) as WhetFloat;
    // }

    // let x = 1.0;
    // let y = 1.0;
    // let mut z = 1.0;
    // for _ in 1..=kilo_whets {
    //     p3(x, y, &mut z);
    // }

    // let mut x = 0.75;
    // for _ in 1..=kilo_whets {
    //     x = sqrt(exp(log(x) / T1));
    // }

    for _ in 0..kilo_whets {
        clear_array(&mut e1);

        //Module 6
        j = j * (k - j) * (l - k);
        k = l * k - (l - j) * k;
        l = (l - k) * (k + j);
        e1[(l - 1) as usize] = (j + k + l) as WhetFloat;
        e1[(k - 1) as usize] = sin(l as WhetFloat);

        //Module 8: Procedure calls

        let mut z = e1[3];
        for inner_loop_var in 1..=10 {
            p3(1.0 * inner_loop_var as WhetFloat, 1.0 + z, &mut z);
        }

        // Module 11: Standard mathematical functions
        z = sqrt(exp(log(e1[(ARRAY_SIZE - 1) as usize]) / T1));
        _sum += z;

        // if fabs(z - VALUE) > TOLERANCE {
        //     _sum *= 2.0; // Forces error at end
        //     j += 1; // Prevents optimization
        // }
    }
}
*/

/*
pub fn small_whetstone2(kilo_whets: usize) {
    for _ in 0..kilo_whets * 1000 {
        asm::nop();
    }
    return;
    let mut sum: f64 = 0.0;

    // Perform a number of "Whetstone instructions"
    for _ in 0..kilo_whets * 1000 {
        // A simple computation
        sum += sqrt(1.0 / (1.0 + (sum * sum)));
    }

    // The result is the number of Kilo-Whetstones executed
    // In a real implementation, you might perform a more complex check
    // For demonstration purposes, we just return kilo_whets
}
*/