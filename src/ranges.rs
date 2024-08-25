
#[derive(Debug)]
struct rle<T> {
    element: Vec<T>,
    freq: Vec<u32>
}

impl rle<Seqnames> {
    fn new() -> Self {
        rle {
            element: Vec::new(),
            freq: Vec::new()
        }
    }


    fn encode_rle(&mut self, var: Vec<Seqnames>) {
        let mut iter: u32 = 1;
        let mut prev_char = var[0];

        for ch in var.iter().skip(1) {
            if *ch != prev_char {
                self.element.push(prev_char);
                self.freq.push(iter);
                iter = 1;
            } else {
                iter += 1;
            }
            prev_char = *ch;
        }
        self.element.push(prev_char);
        self.freq.push(iter);

    }
}


impl rle<Strand> {
    fn new() -> Self {
        rle {
            element: Vec::new(),
            freq: Vec::new()
        }
    }

    fn encode_rle(&mut self, var: Vec<Strand>) {
        let mut iter: u32 = 1;
        let mut prev_char = var[0];

        for ch in var.iter().skip(1) {
            if *ch != prev_char {
                self.element.push(prev_char);
                self.freq.push(iter);
                iter = 1;
            } else {
                iter += 1;
            }
            prev_char = *ch;
        }
        self.element.push(prev_char);
        self.freq.push(iter);
    }
}


#[derive(Debug)]
pub struct Range {
    start: Vec<u64>,
    end: Vec<u64>,
    width: Vec<u64>,
}

impl Range {
    fn new(start: Vec<u64>, end: Vec<u64>) -> Self {
        let mut width: Vec<u64> = vec![0; start.len()];
        for (it, (s, e))  in start.iter().zip(end.iter()).enumerate() {
            width[it] = e - s; 
        }
        Range {
            start,
            end,
            width
        } 
    }
}

#[derive(Debug)]
pub struct Ranges {
    seqnames: rle<Seqnames>,
    range: Vec<Range>,
    strand: rle<Strand>
}

impl Ranges {
    fn new(seqnames: Vec<&str>, range: Vec<Range>, strand: Vec<&str>) -> Self {
        let mut rle_seq: rle<Seqnames> = rle::<Seqnames>::new();
        let mut rle_std: rle<Strand> = rle::<Strand>::new();
        rle_seq.encode_rle(Seqnames::str_to_enum(seqnames));
        rle_std.encode_rle(Strand::str_to_enum(strand));
        Ranges {
            seqnames: rle_seq, 
            range,
            strand: rle_std,
        }
    }

}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Seqnames {
    Chr1,
    Chr2,
    Chr3,
    Chr4,
    Chr5,
    Chr6,
    Chr7,
    Chr8,
    Chr9,
    Chr10,
    Chr11,
    Chr12,
    Chr13,
    Chr14,
    Chr15,
    Chr16,
    Chr17,
    Chr18,
    Chr19,
    Chr20,
    Chr21,
    Chr22,
    ChrX,
    ChrY,
    Unk
}




#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Strand {
    Forward,
    Reverse,
    Unk
}



trait SeqnamesStrand<T> {
    fn str_to_enum(element: Vec<&str>) -> Vec<T>;
}

impl SeqnamesStrand<Seqnames> for Seqnames {
    fn str_to_enum(element: Vec<&str>) -> Vec<Seqnames> {
        let mut element_vec = vec![Seqnames::Chr1; element.len()];

        for (idx, chr) in element.iter().enumerate() {
            element_vec[idx] = match *chr {
                "chr1" => Seqnames::Chr1,
                "chr2" => Seqnames::Chr2,
                "chr3" => Seqnames::Chr3,
                "chr4" => Seqnames::Chr4,
                "chr5" => Seqnames::Chr5,
                "chr6" => Seqnames::Chr6,
                "chr7" => Seqnames::Chr7,
                "chr8" => Seqnames::Chr8,
                "chr9" => Seqnames::Chr9,
                "chr10" => Seqnames::Chr10,
                "chr11" => Seqnames::Chr11,
                "chr12" => Seqnames::Chr12,
                "chr13" => Seqnames::Chr13,
                "chr14" => Seqnames::Chr14,
                "chr15" => Seqnames::Chr15,
                "chr16" => Seqnames::Chr16,
                "chr17" => Seqnames::Chr17,
                "chr18" => Seqnames::Chr18,
                "chr19" => Seqnames::Chr19,
                "chr20" => Seqnames::Chr20,
                "chr21" => Seqnames::Chr21,
                "chr22" => Seqnames::Chr22,
                "chrX" => Seqnames::ChrX,
                "chrY" => Seqnames::ChrY,
                _ => Seqnames::Unk
            }
        }


        return element_vec;
       
    }
}

impl SeqnamesStrand<Strand> for Strand {
    fn str_to_enum(element: Vec<&str>) -> Vec<Strand> {
        let mut element_vec = vec![Strand::Forward; element.len()];

        for (idx, chr) in element.iter().enumerate() {
            element_vec[idx] = match *chr {
                "+" => Strand::Forward,
                "-" => Strand::Reverse,
                _ => Strand::Unk
            }
        }

        return element_vec;
    }
}



#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn init_ranges() {
        let ranges1 = Range::new(vec![0, 1, 2, 3], vec![10, 11, 12, 13]);
        let ranges2 = Range::new(vec![3, 4, 2, 3], vec![14, 11, 12, 13]);
        let ranges3 = Range::new(vec![0, 1, 2, 3], vec![12, 11, 19, 13]);
        let ranges4 = Range::new(vec![0, 4, 4, 3], vec![10, 11, 12, 13]);
        let rs_ranges = Ranges::new(
            vec!["chr1", "chr2", "chr2", "chr3", "cr7"],
            vec![ranges1, ranges2, ranges3, ranges4],
            vec!["-", "-", "-", "+", "s"]
        );
        println!("{:?}", rs_ranges);
    }
}
