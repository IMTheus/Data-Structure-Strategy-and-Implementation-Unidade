// Inclinação x e y
pub struct LinearRegression {
    pub slope: f64,
    pub intercept: f64,
}

// Esse método cria uma nova instância do slope e intercept como 0
impl LinearRegression {
    pub fn new() -> Self {
        LinearRegression {
            slope: 0.0,
            intercept: 0.0,
        }
    }

    // Método fit recebe os dados e faz o cálculo
    pub fn fit(&mut self, x: &[f64], y: &[f64]) {
        let n = x.len();
        if n == 0 || x.len() != y.len() {
            panic!("Entrada inválida: os arrays devem ter o mesmo tamanho e não podem estar vazios");
        }

        let sum_x: f64 = x.iter().sum();
        let sum_y: f64 = y.iter().sum();
        let sum_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
        let sum_x_squared: f64 = x.iter().map(|xi| xi * xi).sum();
        let mean_x = sum_x / n as f64;
        let mean_y = sum_y / n as f64;

        let numerator = sum_xy - n as f64 * mean_x * mean_y;
        let denominator = sum_x_squared - n as f64 * mean_x * mean_x;

        if denominator == 0.0 {
            panic!("Erro: Divisão por zero detectada, os pontos podem estar alinhados verticalmente");
        }

        self.slope = numerator / denominator;
        self.intercept = mean_y - self.slope * mean_x;
    }

    // Método predict recebe um valor x e calcula o valor previsto y
    pub fn predict(&self, x: f64) -> f64 {
        self.intercept + self.slope * x
    }

    // Método para calcular o erro quadrático médio
    pub fn mean_squared_error(&self, x: &[f64], y: &[f64]) -> f64 {
        let n = x.len();
        if n == 0 || x.len() != y.len() {
            panic!("Entrada inválida: os arrays devem ter o mesmo tamanho e não podem estar vazios");
        }
        let mse: f64 = x.iter().zip(y.iter())
            .map(|(xi, yi)| {
                let y_pred = self.predict(*xi);
                (yi - y_pred).powi(2)
            })
            .sum::<f64>() / n as f64;
        mse
    }

    // Método para calcular o R² (coeficiente de determinação)
    
    pub fn r_squared(&self, x: &[f64], y: &[f64]) -> f64 {
        let n = x.len();
        if n == 0 || x.len() != y.len() {
            panic!("Entrada inválida: os arrays devem ter o mesmo tamanho e não podem estar vazios");
        }
        let mean_y: f64 = y.iter().sum::<f64>() / n as f64;
        let total_variance: f64 = y.iter().map(|yi| (yi - mean_y).powi(2)).sum();
        let explained_variance: f64 = x.iter().zip(y.iter())
            .map(|(xi, yi)| {
                let y_pred = self.predict(*xi);
                (y_pred - mean_y).powi(2)
            })
            .sum();
        explained_variance / total_variance
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_fit() {
//         let x = [1.0, 2.0, 3.0, 4.0, 5.0];
//         let y = [2.0, 4.0, 6.0, 8.0, 10.0];
        
//         let mut model = LinearRegression::new();
//         model.fit(&x, &y);

//         assert!((model.slope - 2.0).abs() < 1e-6, "Slope está incorreto");
//         assert!((model.intercept - 0.0).abs() < 1e-6, "Intercepto está incorreto");
//     }

//     #[test]
//     fn test_predict() {
//         let mut model = LinearRegression::new();
//         model.slope = 2.0;
//         model.intercept = 1.0;

//         let predicted = model.predict(3.0);
//         assert!((predicted - 7.0).abs() < 1e-6, "Previsão está incorreta");
//     }

//     #[test]
//     fn test_mse() {
//         let x = [1.0, 2.0, 3.0, 4.0, 5.0];
//         let y = [2.0, 4.0, 6.0, 8.0, 10.0];

//         let mut model = LinearRegression::new();
//         model.fit(&x, &y);
//         let mse = model.mean_squared_error(&x, &y);

//         assert!(mse < 1e-6, "MSE deveria ser próximo de 0");
//     }

//     #[test]
//     fn test_r_squared() {
//         let x = [1.0, 2.0, 3.0, 4.0, 5.0];
//         let y = [2.0, 4.0, 6.0, 8.0, 10.0];

//         let mut model = LinearRegression::new();
//         model.fit(&x, &y);
//         let r2 = model.r_squared(&x, &y);

//         assert!((r2 - 1.0).abs() < 1e-6, "R² deveria ser próximo de 1");
//     }
// }
