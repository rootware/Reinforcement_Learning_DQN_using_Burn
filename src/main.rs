pub mod dqn;
pub mod environment;
pub mod model;
pub mod replay_buffer;
//pub mod training;
pub mod utils;
use burn::tensor::Tensor;
use burn_tch::LibTorchDevice;

//use burn_cuda::CudaDevice;
use dqn::DQN;
use environment::Environment;
use model::Model;
use replay_buffer::ReplayBuffer;
use utils::*;


fn main() {
    //let device = Default::default();
   // let device = CudaDevice::default();
   let device = LibTorchDevice::Cpu;
    let model = model::ModelConfig::new(STATE_SIZE, HIDDEN_SIZE, NUM_ACTIONS)
        .init::<MyAutodiffBackend>(&device);

    let myconfig = MyConfig {
        gamma: 0.99,
        lr: 1.0e-2,
        epsilon: 1.0,
        tau: 0.1,
    };
    
    // Initialize DQN model and optimizer
    let mut dqn_model = DQN::new(model.clone(), model.clone(),ReplayBuffer::new(1000), myconfig.clone());
    
    dqn_model.train(   100, 100);
    println!("zero epsilon policy");
    dqn_model.extract_policy_zero_epsilon();
    println!("best ever policy");
    dqn_model.extract_policy();





}
