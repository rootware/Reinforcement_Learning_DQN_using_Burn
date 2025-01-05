use burn::backend::Wgpu;
use burn::config::{self, Config};
use burn::module::Module;
use burn::nn::loss::Reduction;
use burn::optim::{Adam, AdamConfig, GradientsParams};
use burn::tensor::Tensor;
use burn::train::ValidStep;
use environment::Environment;
use model::Model;




use crate::model::ModelConfig;
use crate::replay_buffer::ReplayBuffer;
use crate::training::TrainingConfig;
use crate::{environment, model}; // For experience replay
type MyBackend = Wgpu<f32, i32>;
// Define a simple neural network for Q-function approximation
pub struct DQN{
    pub env : Environment,
    pub nn_model : Model<MyBackend>,
    pub buffer : ReplayBuffer,
    pub gamma : f64,
}

impl DQN {
    pub fn new(model_arg: Model<MyBackend>, buffer: ReplayBuffer ) -> Self {
        
        DQN {
            env: Environment::new(),
            nn_model: model_arg,
            buffer,
            gamma: 1.0,

        }
    }

    pub fn forward(&self, x: Tensor<MyBackend, 2>) -> Tensor<MyBackend, 2> {
        self.nn_model.forward(x)
    }

    pub fn optimize(&mut self, batch_size: usize) {
        let optimizer: burn::optim::adaptor::OptimizerAdaptor<Adam<_>, _, _> = AdamConfig::new().with_epsilon(0.1).init();
        // Sample a batch of experiences from the replay buffer
        let batch = self.buffer.sample(batch_size);
        // DQN Q-learning update
        for mem in batch {
            // Compute target Q-value

            let tensor_state = Tensor::from(mem.current_state);
            let tensor_next_state = Tensor::from(mem.next_state);
            let done = mem.done;
            let reward = Tensor::from([mem.reward]);
            let action = Tensor::from([mem.action]);

            let next_q_values = self.forward(tensor_next_state);
            let target = if done {
                reward
            } else {
                reward + next_q_values.max().mul_scalar(self.gamma  )
            };

            // Compute Q-value for the current state and action
            let q_values = self.forward(tensor_state.clone());
            let q_value = q_values.select(0, action  );

            let temp  = self.nn_model.clone();
  
            #[derive(Config)]
            pub struct MnistTrainingConfig {
                #[config(default = 10)]
                pub num_epochs: usize,
                #[config(default = 64)]
                pub batch_size: usize,
                #[config(default = 4)]
                pub num_workers: usize,
                #[config(default = 42)]
                pub seed: u64,
                #[config(default = 1e-4)]
                pub lr: f64,
                pub model: ModelConfig,
                pub optimizer: AdamConfig,
            }
            let config_model = self.nn_model;
            let config_optimizer = AdamConfig::new();
           
            let test_input = Tensor::<MyBackend, 2>::from( [[2.0,1.0], [2.0,3.0]]);
            let test_target = Tensor::<MyBackend,2>::from([[2.5,1.2], [2.1,3.1]]);
            let loss = burn::nn::loss::MseLoss::new().forward( test_input, test_target, Reduction::Sum);
            // Gradients for the current backward pass
            let grads = loss.backward();
            // Gradients linked to each parameter of the model.
            let grads = GradientsParams::from_grads(grads, &self.nn_model);
            // Update the model using the optimizer.
            model = optim.step(config.lr, model, grads);



            // Create the model and optimizer.
            let mut model = self.nn_model;
            let mut optim = config_optimizer.init();

        }
    }
}
