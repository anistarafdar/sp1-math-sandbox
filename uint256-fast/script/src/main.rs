use sp1_sdk::prelude::*;
use sp1_sdk::ProverClient;

const ELF: Elf = include_elf!("uint256-program");

#[tokio::main]
async fn main() {
    // Generate proof.
    // utils::setup_tracer();
    sp1_sdk::utils::setup_logger();

    let stdin = SP1Stdin::new();
    let client = ProverClient::from_env().await;
    let pk = client.setup(ELF).await.expect("setup failed");
    let proof = client.prove(&pk, stdin).core().await.expect("proving failed");

    // Verify proof.
    client.verify(&proof, pk.verifying_key(), None).expect("verification failed");
    
    // Test a round trip of proof serialization and deserialization.
    proof.save("proof-with-pis.bin").expect("saving proof failed");
    let deserialized_proof =
        SP1ProofWithPublicValues::load("proof-with-pis.bin").expect("loading proof failed");

    // Verify the deserialized proof.
    client.verify(&deserialized_proof, pk.verifying_key(), None).expect("verification failed");

    println!("successfully generated and verified proof for the program!");
	// Add these 3 lines for cycle count and time
	let duration = proof.stats.total_duration().as_secs_f64();
	let cycles = proof.stats.total_instruction_count();
	println!("Proof generated in {:.2}s with {} cycles", duration, cycles);
}

