extern crate rand;
use self::rand::Rng;

use common::{Float, ONE_MINUS_EPSILON};
use vector::{Point2u, Point2f};
use sampler::{Sampler, GlobalSampler};

/// Max resoltion of one tile.
const K_MAX_RESOLUTION: u32 = 128;
// (x, y, u, v, (u, v)...)
const ARRAY_START_DIM: usize = 4;

pub struct HaltonSampler {
    // General sampler
    samplesPerPixel: usize,

    currentPixel: Point2u,
    currentPixelSampleIndex: usize,

    sampleArray1D: Vec<Box<[Float]>>,
    sampleArray2D: Vec<Box<[Point2f]>>,

    // Next 1d array to be returned
    array1DOffset: usize,
    // Next 2d array to be returned
    array2DOffset: usize,

    // Global sampler
    // Next dimension
    dimension: usize,
    // Index of sample in current pixel
    intervalSampleIndex: usize,

    arrayEndDim: usize,

    // Halton sampler
    baseScale: Point2u,
    baseExp: Point2u,
    sampleStride: usize,
    pixelForOffset: Point2u,
    // First sample in the currentPixel
    offsetForCurrentPixel: usize,
    multiInverse: [usize; 2],
}

impl HaltonSampler {
    pub fn New(samplesPerPixel: usize) -> HaltonSampler {
        unimplemented!()
    }
}

impl Sampler for HaltonSampler {
    fn StartPixel(&mut self, p: Point2u) {
        // General sampler
        self.currentPixel = p;
        self.currentPixelSampleIndex = 0;
        self.array1DOffset = 0;
        self.array2DOffset = 0;

        // Compute 1D array samples
        for i in 0..self.sampleArray1D.len() {
            let nSample = self.sampleArray1D[i].len() * self.samplesPerPixel;
            for j in 0..nSample {
                let index = self.GetIndexForSample(j);
                self.sampleArray1D[i][j] = self.SampleDimension(index, ARRAY_START_DIM + i);
            }
        }

        // Compute 2D array samples
        let mut dim = ARRAY_START_DIM + self.sampleArray1D.len();
        for i in 0..self.sampleArray2D.len() {
            let nSample = self.sampleArray2D[i].len() * self.samplesPerPixel;
            for j in 0..nSample {
                let index = self.GetIndexForSample(j);
                self.sampleArray2D[i][j].X = self.SampleDimension(index, dim);
                self.sampleArray2D[i][j].Y = self.SampleDimension(index, dim + 1);
            }
            dim += 2;
        }

        // Global sampler
        self.dimension = 0;
        self.intervalSampleIndex = self.GetIndexForSample(0);
        self.arrayEndDim = ARRAY_START_DIM
            + self.sampleArray1D.len()
            + self.sampleArray2D.len() * 2;

        debug_assert_eq!(dim, self.arrayEndDim);
    }

    fn StartNextSample(&mut self) -> bool {
        self.currentPixelSampleIndex += 1;
        self.array1DOffset = 0;
        self.array2DOffset = 0;

        self.dimension = 0;
        self.intervalSampleIndex = {
            let nextPixelSampleIndex = self.currentPixelSampleIndex + 1;
            self.GetIndexForSample(nextPixelSampleIndex)
        };

        return self.currentPixelSampleIndex < self.samplesPerPixel;
    }

    fn Get1D(&mut self) -> Float {
        if ARRAY_START_DIM <= self.dimension && self.dimension <= self.arrayEndDim {
            self.dimension = self.arrayEndDim;
        }
        self.dimension += 1;
        return self.SampleDimension(self.intervalSampleIndex, self.dimension);
    }

    fn Get2D(&mut self) -> Point2f {
        if ARRAY_START_DIM <= self.dimension + 1 && self.dimension + 1 <= self.arrayEndDim {
            self.dimension = self.arrayEndDim;
        }
        let p = Point2f::New(
            self.SampleDimension(self.intervalSampleIndex, self.dimension),
            self.SampleDimension(self.intervalSampleIndex, self.dimension + 1),
        );
        self.dimension += 2;
        return p;
    }

    fn Req1DArray(&mut self, n: usize) {
        debug_assert_eq!(self.RoundCount(n), n);
        self.sampleArray1D.push(
            Vec::<Float>
            ::with_capacity((n * self.samplesPerPixel))
            .into_boxed_slice()
        );
    }

    fn Req2DArray(&mut self, n: usize) {
        debug_assert_eq!(self.RoundCount(n), n);
        self.sampleArray2D.push(
            Vec::<Point2f>
            ::with_capacity((n * self.samplesPerPixel))
            .into_boxed_slice()
        );
    }

    fn Get1DArray(&mut self, n: usize) -> Option<&[Float]> {
        if self.array1DOffset == self.sampleArray1D.len() {
            return None;
        }
        debug_assert_eq!(self.sampleArray1D[self.array1DOffset].len(), n);
        debug_assert!(self.currentPixelSampleIndex < self.samplesPerPixel);
        let ret = {
            let i0 = self.currentPixelSampleIndex * n;
            let i1 = i0 + n;
            Some(&self.sampleArray1D[self.array1DOffset][i0..i1])
        };
        self.array1DOffset += 1;
        return ret;
    }

    fn Get2DArray(&mut self, n: usize) -> Option<&[Point2f]> {
        if self.array2DOffset == self.sampleArray2D.len() {
            return None;
        }
        debug_assert_eq!(self.sampleArray2D[self.array2DOffset].len(), n);
        debug_assert!(self.currentPixelSampleIndex < self.samplesPerPixel);
        let ret = {
            let i0 = self.currentPixelSampleIndex * n;
            let i1 = i0 + n;
            Some(&self.sampleArray2D[self.array2DOffset][i0..i1])
        };
        self.array2DOffset += 1;
        return ret;
    }
}

impl GlobalSampler for HaltonSampler {
    fn GetIndexForSample(&mut self, sampleNum: usize) -> usize {
        if self.pixelForOffset != self.currentPixel {
            self.offsetForCurrentPixel = 0;
            if self.sampleStride > 1 {
                let pm = Point2u {
                    X: self.currentPixel.X % K_MAX_RESOLUTION,
                    Y: self.currentPixel.Y % K_MAX_RESOLUTION,
                };

                self.offsetForCurrentPixel += {
                    let dimOffset = InverseRadicalInverse(2, pm.X, self.baseExp.X);
                    dimOffset * (self.sampleStride / self.baseExp.X as usize) * self.multiInverse[0]
                };
                self.offsetForCurrentPixel += {
                    let dimOffset = InverseRadicalInverse(3, pm.Y, self.baseExp.Y);
                    dimOffset * (self.sampleStride / self.baseExp.Y as usize) * self.multiInverse[1]
                };
                self.offsetForCurrentPixel %= self.sampleStride;
            }
            self.pixelForOffset = self.currentPixel;
        }
        return self.offsetForCurrentPixel + sampleNum * self.sampleStride;
    }

    fn SampleDimension(&self, index: usize, d: usize) -> Float {
        let index = index as u32;
        let d = d as u32;
        match d {
            0 => RadicalInverse(0, index >> self.baseExp.X),
            1 => RadicalInverse(1, index / self.baseScale.Y),
            _ => RadicalInverse(d, index),
        }
    }
}

const PRIMES: [usize; 1000] = [2, 3, 5, 7, 11,
    13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
    97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
    173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251,
    257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347,
    349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433,
    439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523,
    541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
    631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727,
    733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827,
    829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937,
    941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031,
    1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103,
    1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187, 1193, 1201,
    1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289,
    1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381,
    1399, 1409, 1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 1459, 1471,
    1481, 1483, 1487, 1489, 1493, 1499, 1511, 1523, 1531, 1543, 1549, 1553,
    1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613, 1619, 1621,
    1627, 1637, 1657, 1663, 1667, 1669, 1693, 1697, 1699, 1709, 1721, 1723,
    1733, 1741, 1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823,
    1831, 1847, 1861, 1867, 1871, 1873, 1877, 1879, 1889, 1901, 1907, 1913,
    1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997, 1999, 2003, 2011,
    2017, 2027, 2029, 2039, 2053, 2063, 2069, 2081, 2083, 2087, 2089, 2099,
    2111, 2113, 2129, 2131, 2137, 2141, 2143, 2153, 2161, 2179, 2203, 2207,
    2213, 2221, 2237, 2239, 2243, 2251, 2267, 2269, 2273, 2281, 2287, 2293,
    2297, 2309, 2311, 2333, 2339, 2341, 2347, 2351, 2357, 2371, 2377, 2381,
    2383, 2389, 2393, 2399, 2411, 2417, 2423, 2437, 2441, 2447, 2459, 2467,
    2473, 2477, 2503, 2521, 2531, 2539, 2543, 2549, 2551, 2557, 2579, 2591,
    2593, 2609, 2617, 2621, 2633, 2647, 2657, 2659, 2663, 2671, 2677, 2683,
    2687, 2689, 2693, 2699, 2707, 2711, 2713, 2719, 2729, 2731, 2741, 2749,
    2753, 2767, 2777, 2789, 2791, 2797, 2801, 2803, 2819, 2833, 2837, 2843,
    2851, 2857, 2861, 2879, 2887, 2897, 2903, 2909, 2917, 2927, 2939, 2953,
    2957, 2963, 2969, 2971, 2999, 3001, 3011, 3019, 3023, 3037, 3041, 3049,
    3061, 3067, 3079, 3083, 3089, 3109, 3119, 3121, 3137, 3163, 3167, 3169,
    3181, 3187, 3191, 3203, 3209, 3217, 3221, 3229, 3251, 3253, 3257, 3259,
    3271, 3299, 3301, 3307, 3313, 3319, 3323, 3329, 3331, 3343, 3347, 3359,
    3361, 3371, 3373, 3389, 3391, 3407, 3413, 3433, 3449, 3457, 3461, 3463,
    3467, 3469, 3491, 3499, 3511, 3517, 3527, 3529, 3533, 3539, 3541, 3547,
    3557, 3559, 3571, 3581, 3583, 3593, 3607, 3613, 3617, 3623, 3631, 3637,
    3643, 3659, 3671, 3673, 3677, 3691, 3697, 3701, 3709, 3719, 3727, 3733,
    3739, 3761, 3767, 3769, 3779, 3793, 3797, 3803, 3821, 3823, 3833, 3847,
    3851, 3853, 3863, 3877, 3881, 3889, 3907, 3911, 3917, 3919, 3923, 3929,
    3931, 3943, 3947, 3967, 3989, 4001, 4003, 4007, 4013, 4019, 4021, 4027,
    4049, 4051, 4057, 4073, 4079, 4091, 4093, 4099, 4111, 4127, 4129, 4133,
    4139, 4153, 4157, 4159, 4177, 4201, 4211, 4217, 4219, 4229, 4231, 4241,
    4243, 4253, 4259, 4261, 4271, 4273, 4283, 4289, 4297, 4327, 4337, 4339,
    4349, 4357, 4363, 4373, 4391, 4397, 4409, 4421, 4423, 4441, 4447, 4451,
    4457, 4463, 4481, 4483, 4493, 4507, 4513, 4517, 4519, 4523, 4547, 4549,
    4561, 4567, 4583, 4591, 4597, 4603, 4621, 4637, 4639, 4643, 4649, 4651,
    4657, 4663, 4673, 4679, 4691, 4703, 4721, 4723, 4729, 4733, 4751, 4759,
    4783, 4787, 4789, 4793, 4799, 4801, 4813, 4817, 4831, 4861, 4871, 4877,
    4889, 4903, 4909, 4919, 4931, 4933, 4937, 4943, 4951, 4957, 4967, 4969,
    4973, 4987, 4993, 4999, 5003, 5009, 5011, 5021, 5023, 5039, 5051, 5059,
    5077, 5081, 5087, 5099, 5101, 5107, 5113, 5119, 5147, 5153, 5167, 5171,
    5179, 5189, 5197, 5209, 5227, 5231, 5233, 5237, 5261, 5273, 5279, 5281,
    5297, 5303, 5309, 5323, 5333, 5347, 5351, 5381, 5387, 5393, 5399, 5407,
    5413, 5417, 5419, 5431, 5437, 5441, 5443, 5449, 5471, 5477, 5479, 5483,
    5501, 5503, 5507, 5519, 5521, 5527, 5531, 5557, 5563, 5569, 5573, 5581,
    5591, 5623, 5639, 5641, 5647, 5651, 5653, 5657, 5659, 5669, 5683, 5689,
    5693, 5701, 5711, 5717, 5737, 5741, 5743, 5749, 5779, 5783, 5791, 5801,
    5807, 5813, 5821, 5827, 5839, 5843, 5849, 5851, 5857, 5861, 5867, 5869,
    5879, 5881, 5897, 5903, 5923, 5927, 5939, 5953, 5981, 5987, 6007, 6011,
    6029, 6037, 6043, 6047, 6053, 6067, 6073, 6079, 6089, 6091, 6101, 6113,
    6121, 6131, 6133, 6143, 6151, 6163, 6173, 6197, 6199, 6203, 6211, 6217,
    6221, 6229, 6247, 6257, 6263, 6269, 6271, 6277, 6287, 6299, 6301, 6311,
    6317, 6323, 6329, 6337, 6343, 6353, 6359, 6361, 6367, 6373, 6379, 6389,
    6397, 6421, 6427, 6449, 6451, 6469, 6473, 6481, 6491, 6521, 6529, 6547,
    6551, 6553, 6563, 6569, 6571, 6577, 6581, 6599, 6607, 6619, 6637, 6653,
    6659, 6661, 6673, 6679, 6689, 6691, 6701, 6703, 6709, 6719, 6733, 6737,
    6761, 6763, 6779, 6781, 6791, 6793, 6803, 6823, 6827, 6829, 6833, 6841,
    6857, 6863, 6869, 6871, 6883, 6899, 6907, 6911, 6917, 6947, 6949, 6959,
    6961, 6967, 6971, 6977, 6983, 6991, 6997, 7001, 7013, 7019, 7027, 7039,
    7043, 7057, 7069, 7079, 7103, 7109, 7121, 7127, 7129, 7151, 7159, 7177,
    7187, 7193, 7207, 7211, 7213, 7219, 7229, 7237, 7243, 7247, 7253, 7283,
    7297, 7307, 7309, 7321, 7331, 7333, 7349, 7351, 7369, 7393, 7411, 7417,
    7433, 7451, 7457, 7459, 7477, 7481, 7487, 7489, 7499, 7507, 7517, 7523,
    7529, 7537, 7541, 7547, 7549, 7559, 7561, 7573, 7577, 7583, 7589, 7591,
    7603, 7607, 7621, 7639, 7643, 7649, 7669, 7673, 7681, 7687, 7691, 7699,
    7703, 7717, 7723, 7727, 7741, 7753, 7757, 7759, 7789, 7793, 7817, 7823,
    7829, 7841, 7853, 7867, 7873, 7877, 7879, 7883, 7901, 7907, 7919];

fn radicalInverse(base: u32, mut a: u32) -> Float {
    let mut x = 0;
    let mut baseN = 1;
    while a > 0 {
        x = x * base + a % base;
        a /= base;
        baseN *= base;
    }
    return ONE_MINUS_EPSILON.min(x as Float / baseN as Float);
}

fn RadicalInverse(baseIndex: u32, a: u32) -> Float {
    return match baseIndex {
        0 => radicalInverse(2, a),
        1 => radicalInverse(3, a),
        2 => radicalInverse(5, a),
        3 => radicalInverse(7, a),
        4 => radicalInverse(11, a),
        5 => radicalInverse(13, a),
        6 => radicalInverse(17, a),
        7 => radicalInverse(19, a),
        8 => radicalInverse(23, a),
        9 => radicalInverse(29, a),
        10 => radicalInverse(31, a),
        11 => radicalInverse(37, a),
        12 => radicalInverse(41, a),
        13 => radicalInverse(43, a),
        14 => radicalInverse(47, a),
        15 => radicalInverse(53, a),
        16 => radicalInverse(59, a),
        17 => radicalInverse(61, a),
        18 => radicalInverse(67, a),
        19 => radicalInverse(71, a),
        20 => radicalInverse(73, a),
        21 => radicalInverse(79, a),
        22 => radicalInverse(83, a),
        23 => radicalInverse(89, a),
        24 => radicalInverse(97, a),
        25 => radicalInverse(101, a),
        26 => radicalInverse(103, a),
        27 => radicalInverse(107, a),
        28 => radicalInverse(109, a),
        29 => radicalInverse(113, a),
        30 => radicalInverse(127, a),
        31 => radicalInverse(131, a),
        32 => radicalInverse(137, a),
        33 => radicalInverse(139, a),
        34 => radicalInverse(149, a),
        35 => radicalInverse(151, a),
        36 => radicalInverse(157, a),
        37 => radicalInverse(163, a),
        38 => radicalInverse(167, a),
        39 => radicalInverse(173, a),
        40 => radicalInverse(179, a),
        41 => radicalInverse(181, a),
        42 => radicalInverse(191, a),
        43 => radicalInverse(193, a),
        44 => radicalInverse(197, a),
        45 => radicalInverse(199, a),
        46 => radicalInverse(211, a),
        47 => radicalInverse(223, a),
        48 => radicalInverse(227, a),
        49 => radicalInverse(229, a),
        50 => radicalInverse(233, a),
        51 => radicalInverse(239, a),
        52 => radicalInverse(241, a),
        53 => radicalInverse(251, a),
        54 => radicalInverse(257, a),
        55 => radicalInverse(263, a),
        56 => radicalInverse(269, a),
        57 => radicalInverse(271, a),
        58 => radicalInverse(277, a),
        59 => radicalInverse(281, a),
        60 => radicalInverse(283, a),
        61 => radicalInverse(293, a),
        62 => radicalInverse(307, a),
        63 => radicalInverse(311, a),
        64 => radicalInverse(313, a),
        65 => radicalInverse(317, a),
        66 => radicalInverse(331, a),
        67 => radicalInverse(337, a),
        68 => radicalInverse(347, a),
        69 => radicalInverse(349, a),
        70 => radicalInverse(353, a),
        71 => radicalInverse(359, a),
        72 => radicalInverse(367, a),
        73 => radicalInverse(373, a),
        74 => radicalInverse(379, a),
        75 => radicalInverse(383, a),
        76 => radicalInverse(389, a),
        77 => radicalInverse(397, a),
        78 => radicalInverse(401, a),
        79 => radicalInverse(409, a),
        80 => radicalInverse(419, a),
        81 => radicalInverse(421, a),
        82 => radicalInverse(431, a),
        83 => radicalInverse(433, a),
        84 => radicalInverse(439, a),
        85 => radicalInverse(443, a),
        86 => radicalInverse(449, a),
        87 => radicalInverse(457, a),
        88 => radicalInverse(461, a),
        89 => radicalInverse(463, a),
        90 => radicalInverse(467, a),
        91 => radicalInverse(479, a),
        92 => radicalInverse(487, a),
        93 => radicalInverse(491, a),
        94 => radicalInverse(499, a),
        95 => radicalInverse(503, a),
        96 => radicalInverse(509, a),
        97 => radicalInverse(521, a),
        98 => radicalInverse(523, a),
        99 => radicalInverse(541, a),
        100 => radicalInverse(547, a),
        101 => radicalInverse(557, a),
        102 => radicalInverse(563, a),
        103 => radicalInverse(569, a),
        104 => radicalInverse(571, a),
        105 => radicalInverse(577, a),
        106 => radicalInverse(587, a),
        107 => radicalInverse(593, a),
        108 => radicalInverse(599, a),
        109 => radicalInverse(601, a),
        110 => radicalInverse(607, a),
        111 => radicalInverse(613, a),
        112 => radicalInverse(617, a),
        113 => radicalInverse(619, a),
        114 => radicalInverse(631, a),
        115 => radicalInverse(641, a),
        116 => radicalInverse(643, a),
        117 => radicalInverse(647, a),
        118 => radicalInverse(653, a),
        119 => radicalInverse(659, a),
        120 => radicalInverse(661, a),
        121 => radicalInverse(673, a),
        122 => radicalInverse(677, a),
        123 => radicalInverse(683, a),
        124 => radicalInverse(691, a),
        125 => radicalInverse(701, a),
        126 => radicalInverse(709, a),
        127 => radicalInverse(719, a),
        128 => radicalInverse(727, a),
        129 => radicalInverse(733, a),
        130 => radicalInverse(739, a),
        131 => radicalInverse(743, a),
        132 => radicalInverse(751, a),
        133 => radicalInverse(757, a),
        134 => radicalInverse(761, a),
        135 => radicalInverse(769, a),
        136 => radicalInverse(773, a),
        137 => radicalInverse(787, a),
        138 => radicalInverse(797, a),
        139 => radicalInverse(809, a),
        140 => radicalInverse(811, a),
        141 => radicalInverse(821, a),
        142 => radicalInverse(823, a),
        143 => radicalInverse(827, a),
        144 => radicalInverse(829, a),
        145 => radicalInverse(839, a),
        146 => radicalInverse(853, a),
        147 => radicalInverse(857, a),
        148 => radicalInverse(859, a),
        149 => radicalInverse(863, a),
        150 => radicalInverse(877, a),
        151 => radicalInverse(881, a),
        152 => radicalInverse(883, a),
        153 => radicalInverse(887, a),
        154 => radicalInverse(907, a),
        155 => radicalInverse(911, a),
        156 => radicalInverse(919, a),
        157 => radicalInverse(929, a),
        158 => radicalInverse(937, a),
        159 => radicalInverse(941, a),
        160 => radicalInverse(947, a),
        161 => radicalInverse(953, a),
        162 => radicalInverse(967, a),
        163 => radicalInverse(971, a),
        164 => radicalInverse(977, a),
        165 => radicalInverse(983, a),
        166 => radicalInverse(991, a),
        167 => radicalInverse(997, a),
        168 => radicalInverse(1009, a),
        169 => radicalInverse(1013, a),
        170 => radicalInverse(1019, a),
        171 => radicalInverse(1021, a),
        172 => radicalInverse(1031, a),
        173 => radicalInverse(1033, a),
        174 => radicalInverse(1039, a),
        175 => radicalInverse(1049, a),
        176 => radicalInverse(1051, a),
        177 => radicalInverse(1061, a),
        178 => radicalInverse(1063, a),
        179 => radicalInverse(1069, a),
        180 => radicalInverse(1087, a),
        181 => radicalInverse(1091, a),
        182 => radicalInverse(1093, a),
        183 => radicalInverse(1097, a),
        184 => radicalInverse(1103, a),
        185 => radicalInverse(1109, a),
        186 => radicalInverse(1117, a),
        187 => radicalInverse(1123, a),
        188 => radicalInverse(1129, a),
        189 => radicalInverse(1151, a),
        190 => radicalInverse(1153, a),
        191 => radicalInverse(1163, a),
        192 => radicalInverse(1171, a),
        193 => radicalInverse(1181, a),
        194 => radicalInverse(1187, a),
        195 => radicalInverse(1193, a),
        196 => radicalInverse(1201, a),
        197 => radicalInverse(1213, a),
        198 => radicalInverse(1217, a),
        199 => radicalInverse(1223, a),
        200 => radicalInverse(1229, a),
        201 => radicalInverse(1231, a),
        202 => radicalInverse(1237, a),
        203 => radicalInverse(1249, a),
        204 => radicalInverse(1259, a),
        205 => radicalInverse(1277, a),
        206 => radicalInverse(1279, a),
        207 => radicalInverse(1283, a),
        208 => radicalInverse(1289, a),
        209 => radicalInverse(1291, a),
        210 => radicalInverse(1297, a),
        211 => radicalInverse(1301, a),
        212 => radicalInverse(1303, a),
        213 => radicalInverse(1307, a),
        214 => radicalInverse(1319, a),
        215 => radicalInverse(1321, a),
        216 => radicalInverse(1327, a),
        217 => radicalInverse(1361, a),
        218 => radicalInverse(1367, a),
        219 => radicalInverse(1373, a),
        220 => radicalInverse(1381, a),
        221 => radicalInverse(1399, a),
        222 => radicalInverse(1409, a),
        223 => radicalInverse(1423, a),
        224 => radicalInverse(1427, a),
        225 => radicalInverse(1429, a),
        226 => radicalInverse(1433, a),
        227 => radicalInverse(1439, a),
        228 => radicalInverse(1447, a),
        229 => radicalInverse(1451, a),
        230 => radicalInverse(1453, a),
        231 => radicalInverse(1459, a),
        232 => radicalInverse(1471, a),
        233 => radicalInverse(1481, a),
        234 => radicalInverse(1483, a),
        235 => radicalInverse(1487, a),
        236 => radicalInverse(1489, a),
        237 => radicalInverse(1493, a),
        238 => radicalInverse(1499, a),
        239 => radicalInverse(1511, a),
        240 => radicalInverse(1523, a),
        241 => radicalInverse(1531, a),
        242 => radicalInverse(1543, a),
        243 => radicalInverse(1549, a),
        244 => radicalInverse(1553, a),
        245 => radicalInverse(1559, a),
        246 => radicalInverse(1567, a),
        247 => radicalInverse(1571, a),
        248 => radicalInverse(1579, a),
        249 => radicalInverse(1583, a),
        250 => radicalInverse(1597, a),
        251 => radicalInverse(1601, a),
        252 => radicalInverse(1607, a),
        253 => radicalInverse(1609, a),
        254 => radicalInverse(1613, a),
        255 => radicalInverse(1619, a),
        256 => radicalInverse(1621, a),
        257 => radicalInverse(1627, a),
        258 => radicalInverse(1637, a),
        259 => radicalInverse(1657, a),
        260 => radicalInverse(1663, a),
        261 => radicalInverse(1667, a),
        262 => radicalInverse(1669, a),
        263 => radicalInverse(1693, a),
        264 => radicalInverse(1697, a),
        265 => radicalInverse(1699, a),
        266 => radicalInverse(1709, a),
        267 => radicalInverse(1721, a),
        268 => radicalInverse(1723, a),
        269 => radicalInverse(1733, a),
        270 => radicalInverse(1741, a),
        271 => radicalInverse(1747, a),
        272 => radicalInverse(1753, a),
        273 => radicalInverse(1759, a),
        274 => radicalInverse(1777, a),
        275 => radicalInverse(1783, a),
        276 => radicalInverse(1787, a),
        277 => radicalInverse(1789, a),
        278 => radicalInverse(1801, a),
        279 => radicalInverse(1811, a),
        280 => radicalInverse(1823, a),
        281 => radicalInverse(1831, a),
        282 => radicalInverse(1847, a),
        283 => radicalInverse(1861, a),
        284 => radicalInverse(1867, a),
        285 => radicalInverse(1871, a),
        286 => radicalInverse(1873, a),
        287 => radicalInverse(1877, a),
        288 => radicalInverse(1879, a),
        289 => radicalInverse(1889, a),
        290 => radicalInverse(1901, a),
        291 => radicalInverse(1907, a),
        292 => radicalInverse(1913, a),
        293 => radicalInverse(1931, a),
        294 => radicalInverse(1933, a),
        295 => radicalInverse(1949, a),
        296 => radicalInverse(1951, a),
        297 => radicalInverse(1973, a),
        298 => radicalInverse(1979, a),
        299 => radicalInverse(1987, a),
        300 => radicalInverse(1993, a),
        301 => radicalInverse(1997, a),
        302 => radicalInverse(1999, a),
        303 => radicalInverse(2003, a),
        304 => radicalInverse(2011, a),
        305 => radicalInverse(2017, a),
        306 => radicalInverse(2027, a),
        307 => radicalInverse(2029, a),
        308 => radicalInverse(2039, a),
        309 => radicalInverse(2053, a),
        310 => radicalInverse(2063, a),
        311 => radicalInverse(2069, a),
        312 => radicalInverse(2081, a),
        313 => radicalInverse(2083, a),
        314 => radicalInverse(2087, a),
        315 => radicalInverse(2089, a),
        316 => radicalInverse(2099, a),
        317 => radicalInverse(2111, a),
        318 => radicalInverse(2113, a),
        319 => radicalInverse(2129, a),
        320 => radicalInverse(2131, a),
        321 => radicalInverse(2137, a),
        322 => radicalInverse(2141, a),
        323 => radicalInverse(2143, a),
        324 => radicalInverse(2153, a),
        325 => radicalInverse(2161, a),
        326 => radicalInverse(2179, a),
        327 => radicalInverse(2203, a),
        328 => radicalInverse(2207, a),
        329 => radicalInverse(2213, a),
        330 => radicalInverse(2221, a),
        331 => radicalInverse(2237, a),
        332 => radicalInverse(2239, a),
        333 => radicalInverse(2243, a),
        334 => radicalInverse(2251, a),
        335 => radicalInverse(2267, a),
        336 => radicalInverse(2269, a),
        337 => radicalInverse(2273, a),
        338 => radicalInverse(2281, a),
        339 => radicalInverse(2287, a),
        340 => radicalInverse(2293, a),
        341 => radicalInverse(2297, a),
        342 => radicalInverse(2309, a),
        343 => radicalInverse(2311, a),
        344 => radicalInverse(2333, a),
        345 => radicalInverse(2339, a),
        346 => radicalInverse(2341, a),
        347 => radicalInverse(2347, a),
        348 => radicalInverse(2351, a),
        349 => radicalInverse(2357, a),
        350 => radicalInverse(2371, a),
        351 => radicalInverse(2377, a),
        352 => radicalInverse(2381, a),
        353 => radicalInverse(2383, a),
        354 => radicalInverse(2389, a),
        355 => radicalInverse(2393, a),
        356 => radicalInverse(2399, a),
        357 => radicalInverse(2411, a),
        358 => radicalInverse(2417, a),
        359 => radicalInverse(2423, a),
        360 => radicalInverse(2437, a),
        361 => radicalInverse(2441, a),
        362 => radicalInverse(2447, a),
        363 => radicalInverse(2459, a),
        364 => radicalInverse(2467, a),
        365 => radicalInverse(2473, a),
        366 => radicalInverse(2477, a),
        367 => radicalInverse(2503, a),
        368 => radicalInverse(2521, a),
        369 => radicalInverse(2531, a),
        370 => radicalInverse(2539, a),
        371 => radicalInverse(2543, a),
        372 => radicalInverse(2549, a),
        373 => radicalInverse(2551, a),
        374 => radicalInverse(2557, a),
        375 => radicalInverse(2579, a),
        376 => radicalInverse(2591, a),
        377 => radicalInverse(2593, a),
        378 => radicalInverse(2609, a),
        379 => radicalInverse(2617, a),
        380 => radicalInverse(2621, a),
        381 => radicalInverse(2633, a),
        382 => radicalInverse(2647, a),
        383 => radicalInverse(2657, a),
        384 => radicalInverse(2659, a),
        385 => radicalInverse(2663, a),
        386 => radicalInverse(2671, a),
        387 => radicalInverse(2677, a),
        388 => radicalInverse(2683, a),
        389 => radicalInverse(2687, a),
        390 => radicalInverse(2689, a),
        391 => radicalInverse(2693, a),
        392 => radicalInverse(2699, a),
        393 => radicalInverse(2707, a),
        394 => radicalInverse(2711, a),
        395 => radicalInverse(2713, a),
        396 => radicalInverse(2719, a),
        397 => radicalInverse(2729, a),
        398 => radicalInverse(2731, a),
        399 => radicalInverse(2741, a),
        400 => radicalInverse(2749, a),
        401 => radicalInverse(2753, a),
        402 => radicalInverse(2767, a),
        403 => radicalInverse(2777, a),
        404 => radicalInverse(2789, a),
        405 => radicalInverse(2791, a),
        406 => radicalInverse(2797, a),
        407 => radicalInverse(2801, a),
        408 => radicalInverse(2803, a),
        409 => radicalInverse(2819, a),
        410 => radicalInverse(2833, a),
        411 => radicalInverse(2837, a),
        412 => radicalInverse(2843, a),
        413 => radicalInverse(2851, a),
        414 => radicalInverse(2857, a),
        415 => radicalInverse(2861, a),
        416 => radicalInverse(2879, a),
        417 => radicalInverse(2887, a),
        418 => radicalInverse(2897, a),
        419 => radicalInverse(2903, a),
        420 => radicalInverse(2909, a),
        421 => radicalInverse(2917, a),
        422 => radicalInverse(2927, a),
        423 => radicalInverse(2939, a),
        424 => radicalInverse(2953, a),
        425 => radicalInverse(2957, a),
        426 => radicalInverse(2963, a),
        427 => radicalInverse(2969, a),
        428 => radicalInverse(2971, a),
        429 => radicalInverse(2999, a),
        430 => radicalInverse(3001, a),
        431 => radicalInverse(3011, a),
        432 => radicalInverse(3019, a),
        433 => radicalInverse(3023, a),
        434 => radicalInverse(3037, a),
        435 => radicalInverse(3041, a),
        436 => radicalInverse(3049, a),
        437 => radicalInverse(3061, a),
        438 => radicalInverse(3067, a),
        439 => radicalInverse(3079, a),
        440 => radicalInverse(3083, a),
        441 => radicalInverse(3089, a),
        442 => radicalInverse(3109, a),
        443 => radicalInverse(3119, a),
        444 => radicalInverse(3121, a),
        445 => radicalInverse(3137, a),
        446 => radicalInverse(3163, a),
        447 => radicalInverse(3167, a),
        448 => radicalInverse(3169, a),
        449 => radicalInverse(3181, a),
        450 => radicalInverse(3187, a),
        451 => radicalInverse(3191, a),
        452 => radicalInverse(3203, a),
        453 => radicalInverse(3209, a),
        454 => radicalInverse(3217, a),
        455 => radicalInverse(3221, a),
        456 => radicalInverse(3229, a),
        457 => radicalInverse(3251, a),
        458 => radicalInverse(3253, a),
        459 => radicalInverse(3257, a),
        460 => radicalInverse(3259, a),
        461 => radicalInverse(3271, a),
        462 => radicalInverse(3299, a),
        463 => radicalInverse(3301, a),
        464 => radicalInverse(3307, a),
        465 => radicalInverse(3313, a),
        466 => radicalInverse(3319, a),
        467 => radicalInverse(3323, a),
        468 => radicalInverse(3329, a),
        469 => radicalInverse(3331, a),
        470 => radicalInverse(3343, a),
        471 => radicalInverse(3347, a),
        472 => radicalInverse(3359, a),
        473 => radicalInverse(3361, a),
        474 => radicalInverse(3371, a),
        475 => radicalInverse(3373, a),
        476 => radicalInverse(3389, a),
        477 => radicalInverse(3391, a),
        478 => radicalInverse(3407, a),
        479 => radicalInverse(3413, a),
        480 => radicalInverse(3433, a),
        481 => radicalInverse(3449, a),
        482 => radicalInverse(3457, a),
        483 => radicalInverse(3461, a),
        484 => radicalInverse(3463, a),
        485 => radicalInverse(3467, a),
        486 => radicalInverse(3469, a),
        487 => radicalInverse(3491, a),
        488 => radicalInverse(3499, a),
        489 => radicalInverse(3511, a),
        490 => radicalInverse(3517, a),
        491 => radicalInverse(3527, a),
        492 => radicalInverse(3529, a),
        493 => radicalInverse(3533, a),
        494 => radicalInverse(3539, a),
        495 => radicalInverse(3541, a),
        496 => radicalInverse(3547, a),
        497 => radicalInverse(3557, a),
        498 => radicalInverse(3559, a),
        499 => radicalInverse(3571, a),
        500 => radicalInverse(3581, a),
        501 => radicalInverse(3583, a),
        502 => radicalInverse(3593, a),
        503 => radicalInverse(3607, a),
        504 => radicalInverse(3613, a),
        505 => radicalInverse(3617, a),
        506 => radicalInverse(3623, a),
        507 => radicalInverse(3631, a),
        508 => radicalInverse(3637, a),
        509 => radicalInverse(3643, a),
        510 => radicalInverse(3659, a),
        511 => radicalInverse(3671, a),
        512 => radicalInverse(3673, a),
        513 => radicalInverse(3677, a),
        514 => radicalInverse(3691, a),
        515 => radicalInverse(3697, a),
        516 => radicalInverse(3701, a),
        517 => radicalInverse(3709, a),
        518 => radicalInverse(3719, a),
        519 => radicalInverse(3727, a),
        520 => radicalInverse(3733, a),
        521 => radicalInverse(3739, a),
        522 => radicalInverse(3761, a),
        523 => radicalInverse(3767, a),
        524 => radicalInverse(3769, a),
        525 => radicalInverse(3779, a),
        526 => radicalInverse(3793, a),
        527 => radicalInverse(3797, a),
        528 => radicalInverse(3803, a),
        529 => radicalInverse(3821, a),
        530 => radicalInverse(3823, a),
        531 => radicalInverse(3833, a),
        532 => radicalInverse(3847, a),
        533 => radicalInverse(3851, a),
        534 => radicalInverse(3853, a),
        535 => radicalInverse(3863, a),
        536 => radicalInverse(3877, a),
        537 => radicalInverse(3881, a),
        538 => radicalInverse(3889, a),
        539 => radicalInverse(3907, a),
        540 => radicalInverse(3911, a),
        541 => radicalInverse(3917, a),
        542 => radicalInverse(3919, a),
        543 => radicalInverse(3923, a),
        544 => radicalInverse(3929, a),
        545 => radicalInverse(3931, a),
        546 => radicalInverse(3943, a),
        547 => radicalInverse(3947, a),
        548 => radicalInverse(3967, a),
        549 => radicalInverse(3989, a),
        550 => radicalInverse(4001, a),
        551 => radicalInverse(4003, a),
        552 => radicalInverse(4007, a),
        553 => radicalInverse(4013, a),
        554 => radicalInverse(4019, a),
        555 => radicalInverse(4021, a),
        556 => radicalInverse(4027, a),
        557 => radicalInverse(4049, a),
        558 => radicalInverse(4051, a),
        559 => radicalInverse(4057, a),
        560 => radicalInverse(4073, a),
        561 => radicalInverse(4079, a),
        562 => radicalInverse(4091, a),
        563 => radicalInverse(4093, a),
        564 => radicalInverse(4099, a),
        565 => radicalInverse(4111, a),
        566 => radicalInverse(4127, a),
        567 => radicalInverse(4129, a),
        568 => radicalInverse(4133, a),
        569 => radicalInverse(4139, a),
        570 => radicalInverse(4153, a),
        571 => radicalInverse(4157, a),
        572 => radicalInverse(4159, a),
        573 => radicalInverse(4177, a),
        574 => radicalInverse(4201, a),
        575 => radicalInverse(4211, a),
        576 => radicalInverse(4217, a),
        577 => radicalInverse(4219, a),
        578 => radicalInverse(4229, a),
        579 => radicalInverse(4231, a),
        580 => radicalInverse(4241, a),
        581 => radicalInverse(4243, a),
        582 => radicalInverse(4253, a),
        583 => radicalInverse(4259, a),
        584 => radicalInverse(4261, a),
        585 => radicalInverse(4271, a),
        586 => radicalInverse(4273, a),
        587 => radicalInverse(4283, a),
        588 => radicalInverse(4289, a),
        589 => radicalInverse(4297, a),
        590 => radicalInverse(4327, a),
        591 => radicalInverse(4337, a),
        592 => radicalInverse(4339, a),
        593 => radicalInverse(4349, a),
        594 => radicalInverse(4357, a),
        595 => radicalInverse(4363, a),
        596 => radicalInverse(4373, a),
        597 => radicalInverse(4391, a),
        598 => radicalInverse(4397, a),
        599 => radicalInverse(4409, a),
        600 => radicalInverse(4421, a),
        601 => radicalInverse(4423, a),
        602 => radicalInverse(4441, a),
        603 => radicalInverse(4447, a),
        604 => radicalInverse(4451, a),
        605 => radicalInverse(4457, a),
        606 => radicalInverse(4463, a),
        607 => radicalInverse(4481, a),
        608 => radicalInverse(4483, a),
        609 => radicalInverse(4493, a),
        610 => radicalInverse(4507, a),
        611 => radicalInverse(4513, a),
        612 => radicalInverse(4517, a),
        613 => radicalInverse(4519, a),
        614 => radicalInverse(4523, a),
        615 => radicalInverse(4547, a),
        616 => radicalInverse(4549, a),
        617 => radicalInverse(4561, a),
        618 => radicalInverse(4567, a),
        619 => radicalInverse(4583, a),
        620 => radicalInverse(4591, a),
        621 => radicalInverse(4597, a),
        622 => radicalInverse(4603, a),
        623 => radicalInverse(4621, a),
        624 => radicalInverse(4637, a),
        625 => radicalInverse(4639, a),
        626 => radicalInverse(4643, a),
        627 => radicalInverse(4649, a),
        628 => radicalInverse(4651, a),
        629 => radicalInverse(4657, a),
        630 => radicalInverse(4663, a),
        631 => radicalInverse(4673, a),
        632 => radicalInverse(4679, a),
        633 => radicalInverse(4691, a),
        634 => radicalInverse(4703, a),
        635 => radicalInverse(4721, a),
        636 => radicalInverse(4723, a),
        637 => radicalInverse(4729, a),
        638 => radicalInverse(4733, a),
        639 => radicalInverse(4751, a),
        640 => radicalInverse(4759, a),
        641 => radicalInverse(4783, a),
        642 => radicalInverse(4787, a),
        643 => radicalInverse(4789, a),
        644 => radicalInverse(4793, a),
        645 => radicalInverse(4799, a),
        646 => radicalInverse(4801, a),
        647 => radicalInverse(4813, a),
        648 => radicalInverse(4817, a),
        649 => radicalInverse(4831, a),
        650 => radicalInverse(4861, a),
        651 => radicalInverse(4871, a),
        652 => radicalInverse(4877, a),
        653 => radicalInverse(4889, a),
        654 => radicalInverse(4903, a),
        655 => radicalInverse(4909, a),
        656 => radicalInverse(4919, a),
        657 => radicalInverse(4931, a),
        658 => radicalInverse(4933, a),
        659 => radicalInverse(4937, a),
        660 => radicalInverse(4943, a),
        661 => radicalInverse(4951, a),
        662 => radicalInverse(4957, a),
        663 => radicalInverse(4967, a),
        664 => radicalInverse(4969, a),
        665 => radicalInverse(4973, a),
        666 => radicalInverse(4987, a),
        667 => radicalInverse(4993, a),
        668 => radicalInverse(4999, a),
        669 => radicalInverse(5003, a),
        670 => radicalInverse(5009, a),
        671 => radicalInverse(5011, a),
        672 => radicalInverse(5021, a),
        673 => radicalInverse(5023, a),
        674 => radicalInverse(5039, a),
        675 => radicalInverse(5051, a),
        676 => radicalInverse(5059, a),
        677 => radicalInverse(5077, a),
        678 => radicalInverse(5081, a),
        679 => radicalInverse(5087, a),
        680 => radicalInverse(5099, a),
        681 => radicalInverse(5101, a),
        682 => radicalInverse(5107, a),
        683 => radicalInverse(5113, a),
        684 => radicalInverse(5119, a),
        685 => radicalInverse(5147, a),
        686 => radicalInverse(5153, a),
        687 => radicalInverse(5167, a),
        688 => radicalInverse(5171, a),
        689 => radicalInverse(5179, a),
        690 => radicalInverse(5189, a),
        691 => radicalInverse(5197, a),
        692 => radicalInverse(5209, a),
        693 => radicalInverse(5227, a),
        694 => radicalInverse(5231, a),
        695 => radicalInverse(5233, a),
        696 => radicalInverse(5237, a),
        697 => radicalInverse(5261, a),
        698 => radicalInverse(5273, a),
        699 => radicalInverse(5279, a),
        700 => radicalInverse(5281, a),
        701 => radicalInverse(5297, a),
        702 => radicalInverse(5303, a),
        703 => radicalInverse(5309, a),
        704 => radicalInverse(5323, a),
        705 => radicalInverse(5333, a),
        706 => radicalInverse(5347, a),
        707 => radicalInverse(5351, a),
        708 => radicalInverse(5381, a),
        709 => radicalInverse(5387, a),
        710 => radicalInverse(5393, a),
        711 => radicalInverse(5399, a),
        712 => radicalInverse(5407, a),
        713 => radicalInverse(5413, a),
        714 => radicalInverse(5417, a),
        715 => radicalInverse(5419, a),
        716 => radicalInverse(5431, a),
        717 => radicalInverse(5437, a),
        718 => radicalInverse(5441, a),
        719 => radicalInverse(5443, a),
        720 => radicalInverse(5449, a),
        721 => radicalInverse(5471, a),
        722 => radicalInverse(5477, a),
        723 => radicalInverse(5479, a),
        724 => radicalInverse(5483, a),
        725 => radicalInverse(5501, a),
        726 => radicalInverse(5503, a),
        727 => radicalInverse(5507, a),
        728 => radicalInverse(5519, a),
        729 => radicalInverse(5521, a),
        730 => radicalInverse(5527, a),
        731 => radicalInverse(5531, a),
        732 => radicalInverse(5557, a),
        733 => radicalInverse(5563, a),
        734 => radicalInverse(5569, a),
        735 => radicalInverse(5573, a),
        736 => radicalInverse(5581, a),
        737 => radicalInverse(5591, a),
        738 => radicalInverse(5623, a),
        739 => radicalInverse(5639, a),
        740 => radicalInverse(5641, a),
        741 => radicalInverse(5647, a),
        742 => radicalInverse(5651, a),
        743 => radicalInverse(5653, a),
        744 => radicalInverse(5657, a),
        745 => radicalInverse(5659, a),
        746 => radicalInverse(5669, a),
        747 => radicalInverse(5683, a),
        748 => radicalInverse(5689, a),
        749 => radicalInverse(5693, a),
        750 => radicalInverse(5701, a),
        751 => radicalInverse(5711, a),
        752 => radicalInverse(5717, a),
        753 => radicalInverse(5737, a),
        754 => radicalInverse(5741, a),
        755 => radicalInverse(5743, a),
        756 => radicalInverse(5749, a),
        757 => radicalInverse(5779, a),
        758 => radicalInverse(5783, a),
        759 => radicalInverse(5791, a),
        760 => radicalInverse(5801, a),
        761 => radicalInverse(5807, a),
        762 => radicalInverse(5813, a),
        763 => radicalInverse(5821, a),
        764 => radicalInverse(5827, a),
        765 => radicalInverse(5839, a),
        766 => radicalInverse(5843, a),
        767 => radicalInverse(5849, a),
        768 => radicalInverse(5851, a),
        769 => radicalInverse(5857, a),
        770 => radicalInverse(5861, a),
        771 => radicalInverse(5867, a),
        772 => radicalInverse(5869, a),
        773 => radicalInverse(5879, a),
        774 => radicalInverse(5881, a),
        775 => radicalInverse(5897, a),
        776 => radicalInverse(5903, a),
        777 => radicalInverse(5923, a),
        778 => radicalInverse(5927, a),
        779 => radicalInverse(5939, a),
        780 => radicalInverse(5953, a),
        781 => radicalInverse(5981, a),
        782 => radicalInverse(5987, a),
        783 => radicalInverse(6007, a),
        784 => radicalInverse(6011, a),
        785 => radicalInverse(6029, a),
        786 => radicalInverse(6037, a),
        787 => radicalInverse(6043, a),
        788 => radicalInverse(6047, a),
        789 => radicalInverse(6053, a),
        790 => radicalInverse(6067, a),
        791 => radicalInverse(6073, a),
        792 => radicalInverse(6079, a),
        793 => radicalInverse(6089, a),
        794 => radicalInverse(6091, a),
        795 => radicalInverse(6101, a),
        796 => radicalInverse(6113, a),
        797 => radicalInverse(6121, a),
        798 => radicalInverse(6131, a),
        799 => radicalInverse(6133, a),
        800 => radicalInverse(6143, a),
        801 => radicalInverse(6151, a),
        802 => radicalInverse(6163, a),
        803 => radicalInverse(6173, a),
        804 => radicalInverse(6197, a),
        805 => radicalInverse(6199, a),
        806 => radicalInverse(6203, a),
        807 => radicalInverse(6211, a),
        808 => radicalInverse(6217, a),
        809 => radicalInverse(6221, a),
        810 => radicalInverse(6229, a),
        811 => radicalInverse(6247, a),
        812 => radicalInverse(6257, a),
        813 => radicalInverse(6263, a),
        814 => radicalInverse(6269, a),
        815 => radicalInverse(6271, a),
        816 => radicalInverse(6277, a),
        817 => radicalInverse(6287, a),
        818 => radicalInverse(6299, a),
        819 => radicalInverse(6301, a),
        820 => radicalInverse(6311, a),
        821 => radicalInverse(6317, a),
        822 => radicalInverse(6323, a),
        823 => radicalInverse(6329, a),
        824 => radicalInverse(6337, a),
        825 => radicalInverse(6343, a),
        826 => radicalInverse(6353, a),
        827 => radicalInverse(6359, a),
        828 => radicalInverse(6361, a),
        829 => radicalInverse(6367, a),
        830 => radicalInverse(6373, a),
        831 => radicalInverse(6379, a),
        832 => radicalInverse(6389, a),
        833 => radicalInverse(6397, a),
        834 => radicalInverse(6421, a),
        835 => radicalInverse(6427, a),
        836 => radicalInverse(6449, a),
        837 => radicalInverse(6451, a),
        838 => radicalInverse(6469, a),
        839 => radicalInverse(6473, a),
        840 => radicalInverse(6481, a),
        841 => radicalInverse(6491, a),
        842 => radicalInverse(6521, a),
        843 => radicalInverse(6529, a),
        844 => radicalInverse(6547, a),
        845 => radicalInverse(6551, a),
        846 => radicalInverse(6553, a),
        847 => radicalInverse(6563, a),
        848 => radicalInverse(6569, a),
        849 => radicalInverse(6571, a),
        850 => radicalInverse(6577, a),
        851 => radicalInverse(6581, a),
        852 => radicalInverse(6599, a),
        853 => radicalInverse(6607, a),
        854 => radicalInverse(6619, a),
        855 => radicalInverse(6637, a),
        856 => radicalInverse(6653, a),
        857 => radicalInverse(6659, a),
        858 => radicalInverse(6661, a),
        859 => radicalInverse(6673, a),
        860 => radicalInverse(6679, a),
        861 => radicalInverse(6689, a),
        862 => radicalInverse(6691, a),
        863 => radicalInverse(6701, a),
        864 => radicalInverse(6703, a),
        865 => radicalInverse(6709, a),
        866 => radicalInverse(6719, a),
        867 => radicalInverse(6733, a),
        868 => radicalInverse(6737, a),
        869 => radicalInverse(6761, a),
        870 => radicalInverse(6763, a),
        871 => radicalInverse(6779, a),
        872 => radicalInverse(6781, a),
        873 => radicalInverse(6791, a),
        874 => radicalInverse(6793, a),
        875 => radicalInverse(6803, a),
        876 => radicalInverse(6823, a),
        877 => radicalInverse(6827, a),
        878 => radicalInverse(6829, a),
        879 => radicalInverse(6833, a),
        880 => radicalInverse(6841, a),
        881 => radicalInverse(6857, a),
        882 => radicalInverse(6863, a),
        883 => radicalInverse(6869, a),
        884 => radicalInverse(6871, a),
        885 => radicalInverse(6883, a),
        886 => radicalInverse(6899, a),
        887 => radicalInverse(6907, a),
        888 => radicalInverse(6911, a),
        889 => radicalInverse(6917, a),
        890 => radicalInverse(6947, a),
        891 => radicalInverse(6949, a),
        892 => radicalInverse(6959, a),
        893 => radicalInverse(6961, a),
        894 => radicalInverse(6967, a),
        895 => radicalInverse(6971, a),
        896 => radicalInverse(6977, a),
        897 => radicalInverse(6983, a),
        898 => radicalInverse(6991, a),
        899 => radicalInverse(6997, a),
        900 => radicalInverse(7001, a),
        901 => radicalInverse(7013, a),
        902 => radicalInverse(7019, a),
        903 => radicalInverse(7027, a),
        904 => radicalInverse(7039, a),
        905 => radicalInverse(7043, a),
        906 => radicalInverse(7057, a),
        907 => radicalInverse(7069, a),
        908 => radicalInverse(7079, a),
        909 => radicalInverse(7103, a),
        910 => radicalInverse(7109, a),
        911 => radicalInverse(7121, a),
        912 => radicalInverse(7127, a),
        913 => radicalInverse(7129, a),
        914 => radicalInverse(7151, a),
        915 => radicalInverse(7159, a),
        916 => radicalInverse(7177, a),
        917 => radicalInverse(7187, a),
        918 => radicalInverse(7193, a),
        919 => radicalInverse(7207, a),
        920 => radicalInverse(7211, a),
        921 => radicalInverse(7213, a),
        922 => radicalInverse(7219, a),
        923 => radicalInverse(7229, a),
        924 => radicalInverse(7237, a),
        925 => radicalInverse(7243, a),
        926 => radicalInverse(7247, a),
        927 => radicalInverse(7253, a),
        928 => radicalInverse(7283, a),
        929 => radicalInverse(7297, a),
        930 => radicalInverse(7307, a),
        931 => radicalInverse(7309, a),
        932 => radicalInverse(7321, a),
        933 => radicalInverse(7331, a),
        934 => radicalInverse(7333, a),
        935 => radicalInverse(7349, a),
        936 => radicalInverse(7351, a),
        937 => radicalInverse(7369, a),
        938 => radicalInverse(7393, a),
        939 => radicalInverse(7411, a),
        940 => radicalInverse(7417, a),
        941 => radicalInverse(7433, a),
        942 => radicalInverse(7451, a),
        943 => radicalInverse(7457, a),
        944 => radicalInverse(7459, a),
        945 => radicalInverse(7477, a),
        946 => radicalInverse(7481, a),
        947 => radicalInverse(7487, a),
        948 => radicalInverse(7489, a),
        949 => radicalInverse(7499, a),
        950 => radicalInverse(7507, a),
        951 => radicalInverse(7517, a),
        952 => radicalInverse(7523, a),
        953 => radicalInverse(7529, a),
        954 => radicalInverse(7537, a),
        955 => radicalInverse(7541, a),
        956 => radicalInverse(7547, a),
        957 => radicalInverse(7549, a),
        958 => radicalInverse(7559, a),
        959 => radicalInverse(7561, a),
        960 => radicalInverse(7573, a),
        961 => radicalInverse(7577, a),
        962 => radicalInverse(7583, a),
        963 => radicalInverse(7589, a),
        964 => radicalInverse(7591, a),
        965 => radicalInverse(7603, a),
        966 => radicalInverse(7607, a),
        967 => radicalInverse(7621, a),
        968 => radicalInverse(7639, a),
        969 => radicalInverse(7643, a),
        970 => radicalInverse(7649, a),
        971 => radicalInverse(7669, a),
        972 => radicalInverse(7673, a),
        973 => radicalInverse(7681, a),
        974 => radicalInverse(7687, a),
        975 => radicalInverse(7691, a),
        976 => radicalInverse(7699, a),
        977 => radicalInverse(7703, a),
        978 => radicalInverse(7717, a),
        979 => radicalInverse(7723, a),
        980 => radicalInverse(7727, a),
        981 => radicalInverse(7741, a),
        982 => radicalInverse(7753, a),
        983 => radicalInverse(7757, a),
        984 => radicalInverse(7759, a),
        985 => radicalInverse(7789, a),
        986 => radicalInverse(7793, a),
        987 => radicalInverse(7817, a),
        988 => radicalInverse(7823, a),
        989 => radicalInverse(7829, a),
        990 => radicalInverse(7841, a),
        991 => radicalInverse(7853, a),
        992 => radicalInverse(7867, a),
        993 => radicalInverse(7873, a),
        994 => radicalInverse(7877, a),
        995 => radicalInverse(7879, a),
        996 => radicalInverse(7883, a),
        997 => radicalInverse(7901, a),
        998 => radicalInverse(7907, a),
        999 => radicalInverse(7919, a),
        1000 => radicalInverse(7927, a),
        1001 => radicalInverse(7933, a),
        1002 => radicalInverse(7937, a),
        1003 => radicalInverse(7949, a),
        1004 => radicalInverse(7951, a),
        1005 => radicalInverse(7963, a),
        1006 => radicalInverse(7993, a),
        1007 => radicalInverse(8009, a),
        1008 => radicalInverse(8011, a),
        1009 => radicalInverse(8017, a),
        1010 => radicalInverse(8039, a),
        1011 => radicalInverse(8053, a),
        1012 => radicalInverse(8059, a),
        1013 => radicalInverse(8069, a),
        1014 => radicalInverse(8081, a),
        1015 => radicalInverse(8087, a),
        1016 => radicalInverse(8089, a),
        1017 => radicalInverse(8093, a),
        1018 => radicalInverse(8101, a),
        1019 => radicalInverse(8111, a),
        1020 => radicalInverse(8117, a),
        1021 => radicalInverse(8123, a),
        1022 => radicalInverse(8147, a),
        1023 => radicalInverse(8161, a),
        _ => panic!("RadicalInverse: baseIndex reaches limit"),
    };
}

fn InverseRadicalInverse(base: u32, mut a: u32, nDigits: u32) -> usize {
    let mut x = 0;
    for _ in 0..nDigits {
        let digit = a % base;
        a /= base;
        x = x * base + digit;
    }
    return x as usize;
}

fn ComputeRadicalInversePermutations() -> Vec<usize> {
    let mut perms = Vec::<usize>::new();
    for p in PRIMES.iter() {
        for i in 0..*p {
            perms.push(i);
        }
        let l = perms.len();
        Shuffle(perms.as_mut_slice(), *p, l, 1);
    }
    return perms;
}

// Shuffle blocks of values of size nDim.
// start: index of the first element of the first block in slice
// count: number of blocks
// nDim: block size
fn Shuffle(s: &mut [usize], start: usize, count: usize, nDim: usize) {
    debug_assert_eq!(start % nDim, 0);

    let mut rng = rand::thread_rng();
    for i in 0..count {
        let other: usize = rng.gen_range(i, count);
        for j in 0..nDim {
            s.swap(
                start + nDim * i + j,
                start + nDim * other + j,
            );
        }
    }
}

fn MultiplicativeInverse(a: i64, n: i64) -> u64 {
    let (x, _) = ExtendedGCD(a, n);
    return (x % n).abs() as u64;
}

// Extended Euclidean algorithm
// ExtendedGCD(a, b) -> (x, y) where
// GCD(a, b) = xa + yb
fn ExtendedGCD(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        return (1, 0);
    } else {
        let d = a / b;
        let (xp, yp) = ExtendedGCD(b, a % b);
        return (yp, xp - (d * yp));
    }
}

#[cfg(test)]
mod sampler_halton_test {
    #[test]
    fn TestRadicalInverse() {
        assert_eq!(super::radicalInverse(10, 0), 0.0);
        assert_eq!(super::radicalInverse(10, 1), 0.1);
        assert_eq!(super::radicalInverse(10, 1234), 0.4321);
    }

    #[test]
    #[should_panic]
    fn TestShuffleInvalidStart() {
        let a: &mut [usize] = &mut [0; 32];
        super::Shuffle(a, 2, 8, 4);
    }

    #[test]
    fn TestExtendedGCD() {
        assert_eq!(super::ExtendedGCD(2, 0), (1, 0));
        assert_eq!(super::ExtendedGCD(2, 4), (1, 0));
        assert_eq!(super::ExtendedGCD(4, 6), (-1, 1));
        assert_eq!(super::ExtendedGCD(8, 6), (1, -1));
        assert_eq!(super::ExtendedGCD(8, 6), (1, -1));
        assert_eq!(super::ExtendedGCD(77, 14), (1, -5));
    }
}
