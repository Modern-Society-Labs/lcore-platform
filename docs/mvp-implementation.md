# MVP Implementation - First Step Code Structure & Development Guide

## 🏗️ **MVP Repository Structure**

This document details the implementation of our **first step** toward the complete Cartesi integration architecture. The MVP provides the foundation that we'll migrate to full Cartesi rollups-node integration.

### **lcore-node-mvp Project Layout**
```
lcore-node-mvp/
├── Cargo.toml                     # Rust workspace configuration
├── README.md                      # MVP-specific documentation
├── docker-compose.yml             # Development environment
├── .env.example                   # Environment configuration template
│
├── src/                           # Main application source
│   ├── main.rs                    # Application entry point
│   ├── config.rs                  # Configuration management
│   ├── error.rs                   # Error handling
│   └── lib.rs                     # Library exports
│
├── api/                           # 🌐 REST API Layer
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs                 # API module exports
│   │   ├── handlers/              # Request handlers
│   │   │   ├── mod.rs
│   │   │   ├── ingestion.rs       # Stage 1 data ingestion endpoints
│   │   │   ├── query.rs           # Stage 2 query processing endpoints
│   │   │   └── health.rs          # Health check endpoints
│   │   ├── middleware/            # API middleware
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs            # DID signature validation
│   │   │   └── logging.rs         # Request logging
│   │   ├── models/                # Request/Response models
│   │   │   ├── mod.rs
│   │   │   ├── ingestion.rs       # Ingestion API models
│   │   │   ├── query.rs           # Query API models
│   │   │   └── common.rs          # Shared models
│   │   └── server.rs              # HTTP server setup
│
├── encryption/                    # 🔒 Dual Encryption Core
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs                 # Encryption module exports
│   │   ├── stage1/                # Stage 1: Data Ingestion Encryption
│   │   │   ├── mod.rs
│   │   │   ├── processor.rs       # Stage 1 processor implementation
│   │   │   ├── aes_encryptor.rs   # AES-256-GCM encryption
│   │   │   └── storage_proof.rs   # RiscZero storage proofs
│   │   ├── stage2/                # Stage 2: Query Processing Encryption
│   │   │   ├── mod.rs
│   │   │   ├── processor.rs       # Stage 2 processor implementation
│   │   │   ├── chacha_encryptor.rs # ChaCha20-Poly1305 encryption
│   │   │   └── query_proof.rs     # RiscZero query proofs
│   │   ├── shared/                # Shared encryption utilities
│   │   │   ├── mod.rs
│   │   │   ├── key_manager.rs     # Cryptographic key management
│   │   │   ├── proof_validator.rs # Cross-stage proof verification
│   │   │   └── types.rs           # Shared encryption types
│   │   └── risc_zero/             # RiscZero proof integration
│   │       ├── mod.rs
│   │       ├── standalone.rs      # Standalone RiscZero client
│   │       ├── proof_generator.rs # Proof generation logic
│   │       └── verifier.rs        # Proof verification
│
├── storage/                       # 🗄️ Database Layer
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs                 # Storage module exports
│   │   ├── sqlite/                # SQLite implementation
│   │   │   ├── mod.rs
│   │   │   ├── connection.rs      # Database connection management
│   │   │   ├── migrations.rs      # Database schema migrations
│   │   │   ├── encrypted_records.rs # Encrypted records operations
│   │   │   ├── device_registry.rs # Device registry operations
│   │   │   └── query_cache.rs     # Query result caching
│   │   ├── models/                # Database models
│   │   │   ├── mod.rs
│   │   │   ├── encrypted_record.rs
│   │   │   ├── device.rs
│   │   │   └── query_result.rs
│   │   └── traits.rs              # Storage abstraction traits
│
├── device/                        # 📱 Device Integration Layer
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs                 # Device module exports
│   │   ├── did/                   # DID validation (adapted from ioID-SDK)
│   │   │   ├── mod.rs
│   │   │   ├── validator.rs       # DID signature validation
│   │   │   ├── resolver.rs        # DID document resolution
│   │   │   └── types.rs           # DID-related types
│   │   ├── jose/                  # JOSE processing (adapted from ioID-SDK)
│   │   │   ├── mod.rs
│   │   │   ├── jws_validator.rs   # JWS signature validation
│   │   │   ├── jwk_manager.rs     # JWK key management
│   │   │   └── types.rs           # JOSE-related types
│   │   ├── registry.rs            # Device registration logic
│   │   └── auth.rs                # Device authentication
│
├── kc_chain/                      # ⛓️ KC-Chain Integration
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs                 # KC-Chain module exports
│   │   ├── client.rs              # KC-Chain RPC client
│   │   ├── contracts/             # Smart contract interfaces
│   │   │   ├── mod.rs
│   │   │   ├── mvp_processor.rs   # MVP IoT Processor contract
│   │   │   └── types.rs           # Contract-related types
│   │   ├── stylus/                # Stylus contract deployment
│   │   │   ├── mod.rs
│   │   │   ├── deployer.rs        # Contract deployment logic
│   │   │   └── config.rs          # Stylus configuration
│   │   └── timer.rs               # Timer service integration
│
├── timer-service/                 # ⏰ Automated Query Service
│   ├── package.json               # Node.js dependencies
│   ├── src/
│   │   ├── index.js               # Service entry point
│   │   ├── timer.js               # Timer logic implementation
│   │   ├── kc-chain-client.js     # KC-Chain interaction
│   │   └── config.js              # Configuration management
│   └── Dockerfile                 # Container configuration
│
├── tests/                         # 🧪 Testing Infrastructure
│   ├── integration/               # Integration tests
│   │   ├── mod.rs
│   │   ├── dual_encryption_test.rs
│   │   ├── api_test.rs
│   │   └── kc_chain_test.rs
│   ├── unit/                      # Unit tests
│   │   ├── encryption_test.rs
│   │   ├── storage_test.rs
│   │   └── device_test.rs
│   ├── fixtures/                  # Test data and fixtures
│   │   ├── device_data.json       # Sample IoT device data
│   │   ├── did_documents.json     # Test DID documents
│   │   └── test_vectors.json      # Encryption test vectors
│   └── utils/                     # Test utilities
│       ├── mod.rs
│       ├── mock_devices.rs        # ioID-SDK device simulators
│       ├── test_db.rs             # Test database setup
│       └── kc_chain_mock.rs       # KC-Chain test environment
│
├── scripts/                       # 📜 Development Scripts
│   ├── setup.sh                   # Development environment setup
│   ├── deploy.sh                  # Deployment automation
│   ├── test-devices.sh            # Device simulation scripts
│   └── kc-chain-deploy.sh         # KC-Chain contract deployment
│
├── docs/                          # 📚 MVP Documentation
│   ├── api.md                     # API documentation
│   ├── encryption.md              # Encryption implementation details
│   ├── deployment.md              # Deployment guide
│   └── migration.md               # Cartesi migration guide
│
└── docker/                        # 🐳 Container Configurations
    ├── Dockerfile.api             # API service container
    ├── Dockerfile.processor       # Encryption processor container
    ├── docker-compose.dev.yml     # Development environment
    └── docker-compose.prod.yml    # Production environment
```

## 🔧 **Core Implementation Components**

### **1. Main Application Entry Point**
```rust
// src/main.rs
use anyhow::Result;
use clap::Parser;
use tracing::{info, instrument};
use tokio;

use lcore_node_mvp::{
    api,
    config::{self, Config},
    encryption,
    storage,
    kc_chain,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(if args.verbose { tracing::Level::DEBUG } else { tracing::Level::INFO })
        .init();
    
    // Load global configuration
    let config = Config::from_file(&args.config)?;
    info!("Loaded configuration from {}", args.config);
    
    // Initialize services with dedicated configs
    // This mapping from global to local config structs was done to
    // decouple the crates from the main lcore-node-mvp crate.
    let storage_config = storage::config::StorageConfig {
        database_url: config.storage.database_url,
    };
    let storage = storage::sqlite::Connection::new(&storage_config).await?;

    let encryption_config = encryption::config::EncryptionConfig {
        stage1_key_path: config.encryption.stage1_key_path,
        stage2_key_path: config.encryption.stage2_key_path,
    };
    let encryption = encryption::DualEncryptionService::new(&encryption_config)?;

    let kc_chain_config = kc_chain::config::KCChainConfig {
        rpc_url: config.kc_chain.rpc_url,
        private_key: config.kc_chain.private_key,
    };
    let kc_chain = kc_chain::Client::new(&kc_chain_config).await?;

    // Start API server
    let api_server = api::server::Server::new(
        config.api.clone(),
        storage,
        encryption,
        kc_chain
    );
    
    info!("Starting lcore-node MVP on port {}", config.api.port);
    api_server.run().await?;
    
    Ok(())
}
```

### **2. Configuration Management**
```rust
// src/config.rs
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub api: ApiConfig,
    pub encryption: EncryptionConfig,
    pub storage: StorageConfig,
    pub kc_chain: KCChainConfig,
    pub risc_zero: RiscZeroConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptionConfig {
    pub stage1_key_path: String,
    pub stage2_key_path: String,
    pub key_rotation_hours: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StorageConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub query_cache_ttl_minutes: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KCChainConfig {
    pub rpc_url: String,
    pub ws_url: String,
    pub chain_id: u64,
    pub private_key: String,
    pub contracts: ContractAddresses,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractAddresses {
    pub mvp_processor: String,
    pub device_registry: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RiscZeroConfig {
    pub dev_mode: bool,
    pub bonsai_api_key: Option<String>,
    pub bonsai_api_url: Option<String>,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(path.as_ref().to_str().unwrap()))
            .add_source(config::Environment::with_prefix("LCORE"))
            .build()?;
        
        settings.try_deserialize()
    }
}
```

### **3. Dual Encryption Service**
```rust
// encryption/src/lib.rs
use anyhow::Result;
use async_trait::async_trait;

pub mod stage1;
pub mod stage2;
pub mod shared;
pub mod risc_zero;

pub use stage1::Stage1Processor;
pub use stage2::Stage2Processor;
pub use shared::{KeyManager, ProofValidator};

#[async_trait]
pub trait EncryptionProcessor {
    type Input;
    type Output;
    
    async fn process(&self, input: Self::Input) -> Result<Self::Output>;
}

pub struct DualEncryptionService {
    stage1: Stage1Processor,
    stage2: Stage2Processor,
    key_manager: KeyManager,
}

impl DualEncryptionService {
    pub fn new(config: &EncryptionConfig) -> Result<Self> {
        let key_manager = KeyManager::new(config)?;
        let stage1 = Stage1Processor::new(&key_manager)?;
        let stage2 = Stage2Processor::new(&key_manager)?;
        
        Ok(Self {
            stage1,
            stage2,
            key_manager,
        })
    }
    
    pub async fn process_ingestion(&self, data: &IoTDeviceData) -> Result<StorageReceipt> {
        self.stage1.process(data).await
    }
    
    pub async fn process_query(&self, query: &QueryRequest) -> Result<EncryptedResponse> {
        self.stage2.process(query).await
    }
}
```

### **4. API Handler Implementation**
```rust
// api/src/handlers/ingestion.rs
use axum::{extract::State, http::StatusCode, response::Json};
use tracing::{info, warn, instrument};
use uuid::Uuid;

use crate::models::ingestion::{IngestRequest, IngestResponse};
use crate::middleware::auth::ValidatedDevice;

#[instrument(skip(state, device))]
pub async fn ingest_device_data(
    State(state): State<AppState>,
    ValidatedDevice(device): ValidatedDevice,
    Json(request): Json<IngestRequest>,
) -> Result<Json<IngestResponse>, (StatusCode, String)> {
    let start_time = std::time::Instant::now();
    
    // Validate request data
    if request.payload.is_empty() {
        warn!("Empty payload received from device {}", device.device_id);
        return Err((StatusCode::BAD_REQUEST, "Empty payload".to_string()));
    }
    
    // Convert to IoT device data
    let device_data = IoTDeviceData {
        device_id: device.device_id,
        device_did: device.device_did,
        payload: base64::decode(&request.payload)
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid base64 payload".to_string()))?,
        signature: request.device_signature,
        timestamp: request.timestamp,
        nonce: request.nonce,
    };
    
    // Process through Stage 1 encryption
    let receipt = state.encryption.process_ingestion(&device_data)
        .await
        .map_err(|e| {
            warn!("Encryption processing failed: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Processing failed".to_string())
        })?;
    
    let processing_time = start_time.elapsed();
    info!(
        "Successfully processed data from device {} in {}ms",
        device.device_id,
        processing_time.as_millis()
    );
    
    Ok(Json(IngestResponse {
        record_id: receipt.record_id,
        proof_hash: receipt.proof_hash,
        status: "encrypted_stored".to_string(),
        processing_time_ms: processing_time.as_millis() as u64,
    }))
}

#[instrument(skip(state))]
pub async fn get_ingestion_status(
    State(state): State<AppState>,
    Path(record_id): Path<u64>,
) -> Result<Json<IngestionStatus>, (StatusCode, String)> {
    let status = state.storage.get_record_status(record_id)
        .await
        .map_err(|e| {
            warn!("Failed to get record status: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
        })?;
    
    match status {
        Some(status) => Ok(Json(status)),
        None => Err((StatusCode::NOT_FOUND, "Record not found".to_string())),
    }
}
```

### **5. Device Authentication Middleware**
```rust
// api/src/middleware/auth.rs
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedDevice {
    pub device_id: String,
    pub device_did: String,
    pub public_key: String,
}

pub async fn validate_device_auth(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // Extract DID signature from header
    let signature = headers
        .get("X-Device-Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing device signature".to_string()))?;
    
    // Extract device ID from header
    let device_id = headers
        .get("X-Device-ID")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing device ID".to_string()))?;
    
    // Validate device signature
    let device = state.device_registry.validate_device_signature(device_id, signature)
        .await
        .map_err(|e| {
            warn!("Device authentication failed: {}", e);
            (StatusCode::UNAUTHORIZED, "Invalid device signature".to_string())
        })?;
    
    debug!("Device {} authenticated successfully", device.device_id);
    
    // Add validated device to request extensions
    request.extensions_mut().insert(device);
    
    Ok(next.run(request).await)
}
```

### **6. Timer Service Implementation**
```javascript
// timer-service/src/timer.js
const cron = require('node-cron');
const { ethers } = require('ethers');
const winston = require('winston');
const axios = require('axios');

class MVPTimerService {
    constructor(config) {
        this.config = config;
        this.provider = new ethers.JsonRpcProvider(config.kcChain.rpcUrl);
        this.wallet = new ethers.Wallet(config.kcChain.privateKey, this.provider);
        
        // MVP IoT Processor contract
        this.contract = new ethers.Contract(
            config.contracts.mvpProcessor,
            MVP_IOT_PROCESSOR_ABI,
            this.wallet
        );
        
        this.logger = winston.createLogger({
            level: 'info',
            format: winston.format.combine(
                winston.format.timestamp(),
                winston.format.json()
            ),
            transports: [
                new winston.transports.Console(),
                new winston.transports.File({ filename: 'timer-service.log' })
            ]
        });
    }
    
    async checkAndTriggerQuery() {
        try {
            // Check if query is due (1-hour intervals for MVP)
            const isDue = await this.contract.isQueryDue();
            
            if (isDue) {
                this.logger.info('Query due - triggering IoT data processing...');
                
                // Trigger automated query
                const tx = await this.contract.triggerAutomatedQuery();
                const receipt = await tx.wait();
                
                this.logger.info(`Query triggered: ${receipt.transactionHash}`);
                
                // Call lcore-node query API
                await this.triggerLcoreNodeQuery();
            } else {
                this.logger.debug('Query not due yet');
            }
        } catch (error) {
            this.logger.error(`Timer service error: ${error.message}`);
        }
    }
    
    async triggerLcoreNodeQuery() {
        try {
            const queryRequest = {
                query_id: `auto_${Date.now()}`,
                device_filters: [], // All devices
                time_range: {
                    start: Date.now() - (60 * 60 * 1000), // Last hour
                    end: Date.now()
                },
                aggregation: 'summary',
                requester_signature: await this.signQueryRequest()
            };
            
            const response = await axios.post(
                `${this.config.lcoreNode.apiUrl}/api/v1/query`,
                queryRequest,
                {
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': `Bearer ${this.config.lcoreNode.apiKey}`
                    }
                }
            );
            
            this.logger.info(`Query processed: ${response.data.query_id}`);
        } catch (error) {
            this.logger.error(`Failed to trigger lcore-node query: ${error.message}`);
        }
    }
    
    start() {
        // Check every minute for query due status
        cron.schedule('* * * * *', () => {
            this.checkAndTriggerQuery();
        });
        
        this.logger.info('MVP Timer Service started - checking every minute');
    }
}

module.exports = MVPTimerService;
```

## 🧪 **Testing Implementation**

### **Integration Test Structure**
```rust
// tests/integration/dual_encryption_test.rs
use anyhow::Result;
use lcore_node_mvp::{
    encryption::DualEncryptionService,
    storage::sqlite::Connection,
    device::{IoTDeviceData, QueryRequest},
};

#[tokio::test]
async fn test_dual_encryption_pipeline() -> Result<()> {
    // Setup test environment
    let config = test_config();
    let storage = Connection::new_in_memory().await?;
    let encryption = DualEncryptionService::new(&config.encryption)?;
    
    // Generate test device data
    let device_data = generate_test_device_data();
    
    // Test Stage 1: Data ingestion encryption
    let receipt = encryption.process_ingestion(&device_data).await?;
    assert!(!receipt.proof_hash.is_empty());
    
    // Verify encrypted data is stored
    let stored_record = storage.get_encrypted_record(receipt.record_id).await?;
    assert!(stored_record.is_some());
    assert_ne!(stored_record.unwrap().encrypted_payload, device_data.payload);
    
    // Test Stage 2: Query processing encryption
    let query_request = QueryRequest {
        query_id: "test_query".to_string(),
        device_filters: vec![device_data.device_id],
        time_range: TimeRange::last_hour(),
        aggregation: AggregationType::Summary,
        requester_signature: "test_signature".to_string(),
    };
    
    let query_response = encryption.process_query(&query_request).await?;
    assert!(!query_response.encrypted_result.is_empty());
    assert!(!query_response.proof.hash().is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_device_authentication() -> Result<()> {
    let device_registry = create_test_device_registry().await?;
    
    // Register test device
    let test_device = generate_test_device();
    device_registry.register_device(&test_device).await?;
    
    // Test valid signature
    let valid_signature = generate_valid_signature(&test_device);
    let validated = device_registry.validate_device_signature(
        &test_device.device_id,
        &valid_signature
    ).await?;
    assert_eq!(validated.device_id, test_device.device_id);
    
    // Test invalid signature
    let invalid_signature = "invalid_signature";
    let result = device_registry.validate_device_signature(
        &test_device.device_id,
        invalid_signature
    ).await;
    assert!(result.is_err());
    
    Ok(())
}
```

## 🚀 **Deployment Configuration**

### **Docker Compose for Development**
```yaml
# docker-compose.dev.yml
version: '3.8'

services:
  lcore-node:
    build:
      context: .
      dockerfile: docker/Dockerfile.api
    ports:
      - "8080:8080"
    environment:
      - LCORE_API__HOST=0.0.0.0
      - LCORE_API__PORT=8080
      - LCORE_STORAGE__DATABASE_URL=sqlite:///data/lcore.db
      - LCORE_KC_CHAIN__RPC_URL=https://rpc.devnet.alchemy.com/7eade438-d743-4dc5-ac64-3480de391200
      - LCORE_KC_CHAIN__CHAIN_ID=1205614515668104
      - LCORE_RISC_ZERO__DEV_MODE=true
    volumes:
      - ./data:/data
      - ./config.toml:/app/config.toml
    depends_on:
      - timer-service
    networks:
      - lcore-network

  timer-service:
    build:
      context: ./timer-service
      dockerfile: Dockerfile
    environment:
      - NODE_ENV=development
      - KC_CHAIN_RPC=https://rpc.devnet.alchemy.com/7eade438-d743-4dc5-ac64-3480de391200
      - KC_CHAIN_ID=1205614515668104
      - LCORE_NODE_API_URL=http://lcore-node:8080
      - QUERY_INTERVAL=3600  # 1 hour for MVP
    depends_on:
      - lcore-node
    networks:
      - lcore-network

  test-device-simulator:
    build:
      context: ./tests/utils
      dockerfile: Dockerfile.simulator
    environment:
      - LCORE_API_URL=http://lcore-node:8080
      - SIMULATION_MODE=continuous
      - DEVICE_COUNT=5
    depends_on:
      - lcore-node
    networks:
      - lcore-network

networks:
  lcore-network:
    driver: bridge

volumes:
  lcore-data:
```

This implementation structure provides a solid foundation for the MVP while maintaining clear abstractions for future Cartesi migration. 