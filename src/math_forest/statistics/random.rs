#![allow(dead_code)]

use rand::prelude::*;
use rand_distr::{Beta, Binomial, Distribution, Exp, Gamma, Normal, Poisson, Uniform};

/// 定义支持的分布类型枚举
/// 提前将 rand_distr 的分布对象实例化，可以极大提高 compute() 的性能
enum DistributionConfig {
    Uniform(Uniform<f64>),
    Normal(Normal<f64>),
    NormalUnit(Normal<f64>),
    Exponential(Exp<f64>),
    Poisson(Poisson<f64>),
    Binomial(Binomial),
    Gamma(Gamma<f64>),
    Beta(Beta<f64>),
}

pub struct RandomMaster {
    config: DistributionConfig,
}

impl RandomMaster {
    // --- 构造函数 (返回 Self) ---

    // 均匀分布 构造函数
    // [from]: 最小值
    // [to]: 最大值
    // [step]: 步长（可选，默认0.1）
    pub fn uniform(from: f64, to: f64) -> Self {
        // 注意：rand_distr 的 Uniform 是 [from, to)
        Self {
            config: DistributionConfig::Uniform(Uniform::new(from, to)),
        }
    }

    pub fn normal_unit() -> Self {
        Self {
            config: DistributionConfig::NormalUnit(Normal::new(0.0, 1.0).unwrap()),
        }
    }

    pub fn normal(mean: f64, stddev: f64) -> Self {
        let dist = Normal::new(mean, stddev).expect("正态分布: 标准差必须大于0");
        Self {
            config: DistributionConfig::Normal(dist),
        }
    }

    pub fn exponential(lambda: f64) -> Self {
        let dist = Exp::new(lambda).expect("指数分布: lambda 必须大于0");
        Self {
            config: DistributionConfig::Exponential(dist),
        }
    }

    pub fn poisson(lambda: f64) -> Self {
        let dist = Poisson::new(lambda).expect("泊松分布: lambda 必须大于0");
        Self {
            config: DistributionConfig::Poisson(dist),
        }
    }

    pub fn binomial(n: u64, p: f64) -> Self {
        let dist = Binomial::new(n, p).expect("二项分布: 参数无效");
        Self {
            config: DistributionConfig::Binomial(dist),
        }
    }

    pub fn gamma(shape: f64, scale: f64) -> Self {
        let dist = Gamma::new(shape, scale).expect("伽马分布: 参数必须大于0");
        Self {
            config: DistributionConfig::Gamma(dist),
        }
    }

    pub fn beta(alpha: f64, beta: f64) -> Self {
        let dist = Beta::new(alpha, beta).expect("贝塔分布: 参数必须大于0");
        Self {
            config: DistributionConfig::Beta(dist),
        }
    }

    // --- 核心方法 ---

    /// 执行采样计算
    pub fn compute(&self) -> f64 {
        let mut rng = thread_rng();
        match &self.config {
            DistributionConfig::Uniform(d) => d.sample(&mut rng),
            DistributionConfig::Normal(d) => d.sample(&mut rng),
            DistributionConfig::NormalUnit(d) => d.sample(&mut rng),
            DistributionConfig::Exponential(d) => d.sample(&mut rng),
            DistributionConfig::Poisson(d) => d.sample(&mut rng),
            DistributionConfig::Binomial(d) => d.sample(&mut rng) as f64,
            DistributionConfig::Gamma(d) => d.sample(&mut rng),
            DistributionConfig::Beta(d) => d.sample(&mut rng),
        }
    }

    // --- 静态工具方法 (Static Methods) ---

    pub fn rand() -> f64 {
        rand::random()
    }

    pub fn random_int(max: i32) -> i32 {
        thread_rng().gen_range(0..max)
    }

    pub fn random_in_range(min: f64, max: f64) -> f64 {
        thread_rng().gen_range(min..max)
    }
}

//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform() {
        let from = 0.0;
        let to = 10.0;
        let master = RandomMaster::uniform(from, to);
        for _ in 0..100 {
            let val = master.compute();
            print!("-> {}", val);
            assert!(
                val >= from && val < to,
                "Uniform value {} out of range",
                val
            );
        }
    }

    #[test]
    fn test_normal_unit() {
        let master = RandomMaster::normal_unit();
        // 标准正态分布虽然理论上范围是 (-inf, +inf)，
        // 但我们可以测试多次生成是否会崩溃，并观察其大致范围
        let val = master.compute();
        println!("Normal Unit Sample: {}", val);
    }

    #[test]
    fn test_binomial() {
        let n = 10;
        let p = 0.5;
        let master = RandomMaster::binomial(n, p);
        for _ in 0..100 {
            let val = master.compute();
            // 二项分布的结果不应超过试验次数 n
            assert!(val >= 0.0 && val <= n as f64);
            // 二项分布应该是整数值（虽然我们返回 f64）
            assert_eq!(val, val.floor());
        }
    }

    #[test]
    fn test_static_methods() {
        // 测试 rand()
        let r = RandomMaster::rand();
        assert!(r >= 0.0 && r < 1.0);

        // 测试 random_int
        let i = RandomMaster::random_int(5);
        assert!(i >= 0 && i < 5);
    }

    #[test]
    #[should_panic(expected = "正态分布: 标准差必须大于0")]
    fn test_invalid_normal_params() {
        // 测试非法参数是否触发 panic
        RandomMaster::normal(5.0, -1.0);
    }

    #[test]
    fn test_beta_distribution() {
        // Beta 分布的值域必须在 [0, 1]
        let master = RandomMaster::beta(0.5, 0.5);
        for _ in 0..100 {
            let val = master.compute();
            assert!(val >= 0.0 && val <= 1.0);
        }
    }
}
