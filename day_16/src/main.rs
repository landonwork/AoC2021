use hex::decode;

mod tests {
    use super::*;

    #[test]
    fn one_byte() {
        let x = vec![255u8];
        let y = vec![0u8];
        let z = vec![85u8]; // 0b01010101

        assert_eq!(read_bits(&x, &mut 3, 4), 3);
        assert_eq!(read_bits(&y, &mut 3, 4), 0);
        assert_eq!(read_bits(&z, &mut 3, 4), 2);
    }

    #[test]
    fn two_bytes() {
        let x = vec![255u8, 255];
        let y = vec![0u8, 0];
        let z = vec![85u8, 85]; // 0b01010101

        assert_eq!(read_bits(&x, &mut 6, 9), 15);
        assert_eq!(read_bits(&y, &mut 6, 9), 0);
        assert_eq!(read_bits(&z, &mut 6, 9), 5);
    }

    #[test]
    fn one_packet() {
        let v: Vec<u8> = vec![0b11010010, 0b11111110, 0b00101000];
        let mut pos = 0;
        let x = sum_versions(&v, &mut pos);
        assert_eq!(x, 6);
    }

    #[test]
    fn three_packets() {
        let v: Vec<u8> = decode("38006F45291200").unwrap();
        let mut pos = 0;
        let x = sum_versions(&v, &mut pos);
        assert_eq!(x, 9);
    }

    #[test]
    fn four_packets() {
        let v: Vec<u8> = decode("EE00D40C823060").unwrap();
        let mut pos = 0;
        let x = sum_versions(&v, &mut pos);
        assert_eq!(x, 14);
    }

    #[test]
    fn part1_examples() {
        let v1: Vec<u8> = decode("8A004A801A8002F478").unwrap();
        let v2: Vec<u8> = decode("620080001611562C8802118E34").unwrap();
        let v3: Vec<u8> = decode("C0015000016115A2E0802F182340").unwrap();
        let mut pos = 0;

        let mut x = sum_versions(&v1, &mut pos);
        assert_eq!(x, 16);

        pos = 0;
        x = sum_versions(&v2, &mut pos);
        assert_eq!(x, 12);

        pos = 0;
        x = sum_versions(&v3, &mut pos);
        assert_eq!(x, 23);
    }

    #[test]
    fn addition() {
        let v: Vec<u8> = decode("C200B40A82").unwrap();
        let mut pos = 0;
        let x = evaluate_packet(&v, &mut pos);
        assert_eq!(x, 3);
    }

    #[test]
    fn multiplication() {
        let v: Vec<u8> = decode("04005AC33890").unwrap();
        let mut pos = 0;
        let x = evaluate_packet(&v, &mut pos);
        assert_eq!(x, 54);
    }

    #[test]
    fn minimum() {
        let v: Vec<u8> = decode("880086C3E88112").unwrap();
        let mut pos = 0;
        let x = evaluate_packet(&v, &mut pos);
        assert_eq!(x, 7);
    }

    #[test]
    fn maximum() {
        let v: Vec<u8> = decode("CE00C43D881120").unwrap();
        let mut pos = 0;
        let x = evaluate_packet(&v, &mut pos);
        assert_eq!(x, 9);
    }

    #[test]
    fn greater_than() {
        let v: Vec<u8> = decode("F600BC2D8F").unwrap();
        let mut pos = 0;
        let x = evaluate_packet(&v, &mut pos);
        assert_eq!(x, 0);
    }

    #[test]
    fn less_than() {
        let v: Vec<u8> = decode("D8005AC2A8F0").unwrap();
        let mut pos = 0;
        let x = evaluate_packet(&v, &mut pos);
        assert_eq!(x, 1);
    }

    #[test]
    fn equal_to() {
        let v: Vec<u8> = decode("9C005AC2F8F0").unwrap();
        let mut pos = 0;
        let x = evaluate_packet(&v, &mut pos);
        assert_eq!(x, 0);
    }

    #[test]
    fn all_together() {
        let v: Vec<u8> = decode("9C0141080250320F1802104A08").unwrap();
        let mut pos = 0;
        let x = evaluate_packet(&v, &mut pos);
        assert_eq!(x, 1);
    }
}

fn main() {
    let raw = std::fs::read_to_string("input.txt").unwrap();
    let v: Vec<u8> = decode(raw.trim()).unwrap();

    println!("{:?}", part1(v.clone()));
    println!("{}", part2(v));
}

fn part1(v: Vec<u8>) -> usize {
    let mut pos = 0usize;

    sum_versions(&v, &mut pos)
}

fn sum_versions(v: &Vec<u8>, pos: &mut usize) -> usize {
    let mut ver = read_bits(v, pos, *pos + 2);
    let typ = read_bits(v, pos, *pos + 2);

    if typ == 4 {
        let mut cont: Option<usize> = None;
        let mut lit: usize = 0;
        while cont != Some(0) {
            cont = Some(read_bits(v, pos, *pos));
            lit = (lit << 4) + read_bits(v, pos, *pos+3) as usize;
        }
    } else {
        let len_typ = read_bits(v, pos, *pos);
        if len_typ == 0 {
            let n_bits = read_bits(v, pos, *pos+14);
            let ebit = *pos + n_bits - 1;
            while *pos <= ebit {
                ver += sum_versions(v, pos);
            }
        } else {
            let n_packets = read_bits(v, pos, *pos+10);
            for _ in 0..n_packets {
                ver += sum_versions(v, pos);
            }
        }
    }

    ver
}

fn part2(v: Vec<u8>) -> usize {
    let mut pos = 0usize;

    evaluate_packet(&v, &mut pos)
}

fn evaluate_packet(v: &Vec<u8>, pos: &mut usize) -> usize {
    let _ver = read_bits(v, pos, *pos + 2);
    let typ = read_bits(v, pos, *pos + 2);
    let val = 0;

    if typ == 4 {
        let mut cont: Option<usize> = None;
        let mut lit: usize = 0;
        while cont != Some(0) {
            cont = Some(read_bits(v, pos, *pos));
            lit = (lit << 4) + read_bits(v, pos, *pos+3) as usize;
        }
        lit
    } else {

        let len_typ = read_bits(v, pos, *pos);
        let mut vals: Vec<usize> = Vec::new();

        if len_typ == 0 {

            let n_bits = read_bits(v, pos, *pos+14);
            let ebit = *pos + n_bits - 1;

            while *pos <= ebit {
                vals.push(evaluate_packet(v, pos));
            }

        } else {

            let n_packets = read_bits(v, pos, *pos+10);

            for _ in 0..n_packets {
                vals.push(evaluate_packet(v, pos));
            }
            
        }

        if typ == 0 {
            vals.into_iter().sum::<usize>()
        } else if typ == 1 {
            vals.into_iter().product::<usize>()
        } else if typ == 2 {
            vals.into_iter().min().unwrap()
        } else if typ == 3 {
            vals.into_iter().max().unwrap()
        } else if typ == 5 {
            (vals[0] > vals[1]) as usize
        } else if typ == 6 {
            (vals[0] < vals[1]) as usize
        } else if typ == 7 {
            (vals[0] == vals[1]) as usize
        } else { 10 }
    }
}

fn read_bits(v: &Vec<u8>, sbit: &mut usize, ebit: usize) -> usize {

    let mut value = 0usize;
    
    while *sbit <= ebit {
        let dist = 7 - *sbit % 8;
        value = (value << 1) + ((v[*sbit >> 3] & 1 << dist) >> dist) as usize;
        *sbit += 1;
    }
    value
}
