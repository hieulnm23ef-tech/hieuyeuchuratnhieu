#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, Symbol, Address, Vec
};

// ===== STORAGE KEY =====
const JOBS_KEY: Symbol = symbol_short!("JOBS");

// ===== DATA STRUCT =====
#[derive(Clone)]
#[contracttype]
pub struct Job {
    pub job_id: u64,
    pub client: Address,
    pub freelancer: Address,
    pub amount: i128,
    pub paid: bool,
}

// ===== CONTRACT =====
#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {

    // =========================
    // CREATE JOB
    // =========================
    pub fn create_job(
        env: Env,
        job_id: u64,
        client: Address,
        freelancer: Address,
        amount: i128,
    ) {
        client.require_auth();

        let mut jobs: Vec<Job> = env
            .storage()
            .instance()
            .get(&JOBS_KEY)
            .unwrap_or(Vec::new(&env));

        let job = Job {
            job_id,
            client,
            freelancer,
            amount,
            paid: false,
        };

        jobs.push_back(job);
        env.storage().instance().set(&JOBS_KEY, &jobs);
    }

    // =========================
    // MARK AS PAID
    // =========================
    pub fn mark_paid(env: Env, job_id: u64) {

        let mut jobs: Vec<Job> = env
            .storage()
            .instance()
            .get(&JOBS_KEY)
            .unwrap();

        for i in 0..jobs.len() {
            let mut job = jobs.get(i).unwrap();

            if job.job_id == job_id {

                // chỉ client xác nhận đã trả
                job.client.require_auth();

                if job.paid {
                    panic!("Already paid");
                }

                job.paid = true;

                jobs.set(i, job);
                env.storage().instance().set(&JOBS_KEY, &jobs);
                return;
            }
        }

        panic!("Job not found");
    }

    // =========================
    // GET JOB
    // =========================
    pub fn get_job(env: Env, job_id: u64) -> Job {
        let jobs: Vec<Job> = env
            .storage()
            .instance()
            .get(&JOBS_KEY)
            .unwrap();

        for job in jobs.iter() {
            if job.job_id == job_id {
                return job;
            }
        }

        panic!("Job not found");
    }

    // =========================
    // LIST JOBS
    // =========================
    pub fn list_jobs(env: Env) -> Vec<Job> {
        env.storage()
            .instance()
            .get(&JOBS_KEY)
            .unwrap_or(Vec::new(&env))
    }
}