// imports

mod regression; 

use regression::LinearRegression; 

//  Cria um novo modelo de regressão linear e o armazena na variável model mutavel

fn main() {
    let mut model = LinearRegression::new();

    let x = vec![1.0, 2.0, 3.0];
    let y = vec![2.0, 4.0, 6.0];

    // Treinando o modelo com os dados
    model.fit(&x, &y);

    // Exibindo o valor da inclinação (slope) e intercepto
    println!("Inclinação: {}", model.slope);
    println!("Intercepto: {}", model.intercept);

    // Fazendo uma previsão
    let prediction = model.predict(5.0);
    println!("Previsão para x = 5: {}", prediction);

    // Calculando o erro quadrático médio (MSE)
    let mse = model.mean_squared_error(&x, &y);
    println!("Erro Quadrático Médio (MSE): {}", mse);

    // Calculando o R²
    let r2 = model.r_squared(&x, &y);
    println!("R²: {}", r2);
}

#[cfg(test)]
mod tests {
    use super::*; //  LinearRegression aqui

    #[test]
    fn test_fit() {
        let mut model = LinearRegression::new();

        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];

        model.fit(&x, &y);

        assert!((model.slope - 2.0).abs() < 1e-6, "Slope está incorreto");
        assert!((model.intercept - 0.0).abs() < 1e-6, "Intercepto está incorreto");
    }

    #[test]
    fn test_mse() {
        let mut model = LinearRegression::new();

        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];

        model.fit(&x, &y);

        let mse = model.mean_squared_error(&x, &y);
        assert!((mse - 0.0).abs() < 1e-6, "MSE está incorreto");
    }

    #[test]
    fn test_predict() {
        let mut model = LinearRegression::new();

        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];

        model.fit(&x, &y);

        let prediction = model.predict(5.0);
        assert!((prediction - 10.0).abs() < 1e-6, "Previsão está incorreta");
    }

    #[test]
    fn test_non_linear_data() {
        let mut model = LinearRegression::new();

        let x = vec![1.0, 2.0, 3.0, 4.0];
        let y = vec![1.0, 4.0, 9.0, 16.0]; // Dados quadráticos (não lineares)

        model.fit(&x, &y);

        let prediction = model.predict(5.0);
       
        assert!((prediction - 25.0).abs() < 10.0, "Previsão para 5 está incorreta");
    }

    #[test]
    fn test_r_squared() {
        let mut model = LinearRegression::new();

        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];

        model.fit(&x, &y);

        let r_squared = model.r_squared(&x, &y);
        assert!((r_squared - 1.0).abs() < 1e-6, "R² está incorreto");
    }
}
