# Cartesi Integration Plan - Complete Migration Implementation

## 🎯 **Integration Overview**

This document outlines the complete technical implementation for migrating from the MVP standalone dual encryption architecture to full Cartesi rollups-node integration. This is the planned progression after MVP completion.

## 🏗️ **Technical Integration Architecture**

### **Phase 1: Infrastructure Setup (Weeks 9-12)**

#### **Cartesi Node Deployment**

```bash
# Cartesi infrastructure setup
docker-compose up -d cartesi-node
docker-compose up -d rollups-node

# Verify Cartesi VM functionality
curl http://localhost:8080/health
curl http://localhost:5004/graphql
```

#### **VM Environment Configuration**

```dockerfile
# Cartesi machine configuration
FROM cartesi/machine-emulator:latest

# Install dependencies for lcore-node
RUN apt-get update && apt-get install -y \
    sqlite3 \
    libssl-dev \
    pkg-config \
    build-essential

# Copy lcore-node binary into VM
COPY target/release/lcore-node /opt/lcore-node
COPY config/cartesi-config.toml /opt/config.toml

# Set up SQLite database in VM
RUN sqlite3 /opt/lcore.db < schema.sql

CMD ["/opt/lcore-node", "--config", "/opt/config.toml"]
```

### **Phase 2: Application Migration (Weeks 13-16)**

#### **Storage Layer Migration**

```rust
// MVP → Cartesi storage migration
pub struct CartesiStorageMigration {
    mvp_db: SqliteConnection,
    cartesi_client: CartesiVMClient,
}

impl CartesiStorageMigration {
    pub async fn migrate_encrypted_data(&self) -> Result<()> {
        // 1. Export all encrypted records from MVP SQLite
        let records = self.mvp_db.export_all_encrypted_records().await?;
        
        // 2. Batch insert into Cartesi VM SQLite
        for batch in records.chunks(100) {
            let insert_sql = self.generate_batch_insert_sql(batch);
            self.cartesi_client.execute_sql(&insert_sql).await?;
        }
        
        // 3. Verify data integrity
        self.verify_migration_integrity().await?;
        
        Ok(())
    }
    
    async fn verify_migration_integrity(&self) -> Result<()> {
        let mvp_count = self.mvp_db.count_records().await?;
        let cartesi_count = self.cartesi_client.count_records().await?;
        
        if mvp_count != cartesi_count {
            return Err(anyhow!("Migration integrity check failed"));
        }
        
        Ok(())
    }
}
```

#### **API Gateway Migration**

```rust
// Cartesi rollups integration
pub struct CartesiAPIGateway {
    rollups_client: RollupsClient,
    graphql_client: GraphQLClient,
}

impl CartesiAPIGateway {
    pub async fn submit_iot_data(&self, data: &IoTData) -> Result<InputIndex> {
        // Convert IoT data to Cartesi input format
        let input = CartesiInput {
            payload: serde_json::to_vec(data)?,
            metadata: InputMetadata {
                msg_sender: Address::zero(), // Anonymous IoT data
                timestamp: SystemTime::now(),
            },
        };
        
        // Submit to Cartesi VM via rollups API
        let result = self.rollups_client.submit_input(input).await?;
        
        Ok(result.input_index)
    }
    
    pub async fn query_processed_results(&self, query: &QueryRequest) -> Result<Vec<Notice>> {
        // Query Cartesi VM results via GraphQL
        let notices = self.graphql_client.query_notices(NoticeFilter {
            input_index: None,
            notice_index: None,
        }).await?;
        
        // Filter notices based on query parameters
        let filtered = notices.into_iter()
            .filter(|notice| self.matches_query_filter(notice, query))
            .collect();
        
        Ok(filtered)
    }
}
```

### **Phase 3: Proof System Integration (Weeks 17-18)**

#### **RiscZero Integration in Cartesi VM**

```rust
// Enhanced proof generation within Cartesi environment
pub struct CartesiProofGenerator {
    vm_context: CartesiVMContext,
    risc_zero_config: RiscZeroConfig,
}

impl CartesiProofGenerator {
    pub async fn generate_deterministic_proof(&self, data: &ProcessingResult) -> Result<Proof> {
        // Generate proof within Cartesi VM for deterministic execution
        let proof_input = ProofInput {
            data: data.clone(),
            vm_state_hash: self.vm_context.get_state_hash(),
            execution_environment: "cartesi-vm".to_string(),
        };
        
        // Use RiscZero within deterministic environment
        let proof = self.risc_zero_config.generate_proof(proof_input).await?;
        
        // Verify proof can be independently verified
        self.verify_proof_determinism(&proof).await?;
        
        Ok(proof)
    }
    
    async fn verify_proof_determinism(&self, proof: &Proof) -> Result<()> {
        // Verify the proof is deterministic and reproducible
        let verification_result = self.risc_zero_config.verify_proof(proof).await?;
        
        if !verification_result.is_deterministic {
            return Err(anyhow!("Proof is not deterministic"));
        }
        
        Ok(())
    }
}
```

### **Phase 4: Smart Contract Integration (Weeks 19-20)**

#### **Cartesi Settlement Contracts**

```solidity
// Enhanced settlement with Cartesi verification
contract CartesiIoTProcessor {
    ICartesi public cartesi;
    mapping(bytes32 => ProcessingResult) public results;
    
    struct ProcessingResult {
        bytes32 dataHash;
        bytes encryptedResult;
        bytes32 cartesiProofHash;
        uint256 epoch;
        bool verified;
    }
    
    function submitCartesiResult(
        bytes32 dataHash,
        bytes calldata encryptedResult,
        bytes32 cartesiProofHash,
        uint256 epoch
    ) external {
        // On-chain proof verification is removed.
        // The contract relies on the Cartesi fraud-proof system for dispute resolution.
        results[dataHash] = ProcessingResult({
            dataHash: dataHash,
            encryptedResult: encryptedResult,
            cartesiProofHash: cartesiProofHash,
            epoch: epoch,
            verified: true
        });
        
        emit CartesiResultVerified(dataHash, epoch);
    }
    
    function disputeResult(bytes32 dataHash, bytes calldata counterProof) external {
        ProcessingResult storage result = results[dataHash];
        require(result.verified, "Result not found");
        
        // Initiate dispute resolution via Cartesi
        cartesi.initiateDispute(result.cartesiProofHash, counterProof);
        
        // Mark result as disputed
        result.verified = false;
        
        emit ResultDisputed(dataHash);
    }
}
```

## 📋 **Migration Implementation Checklist**

### **Infrastructure Migration**

- [ ] **Cartesi node and rollups infrastructure deployed**
- [ ] **VM environment configured with all dependencies**
- [ ] **Network connectivity to KC-Chain established**
- [ ] **GraphQL and HTTP APIs accessible**

### **Application Migration**

- [ ] **All encrypted data successfully migrated to Cartesi VM**
- [ ] **rollups-node GraphQL API integrated**
- [ ] **RiscZero working within Cartesi environment**
- [ ] **API gateway updated for Cartesi inputs/outputs**

### **Smart Contract Deployment**

- [ ] **Enhanced settlement contracts deployed to KC-Chain**
- [ ] **Cartesi proof verification working**
- [ ] **Dispute resolution mechanisms functional**
- [ ] **Timer service updated for Cartesi triggers**

### **Testing & Validation**

- [ ] **End-to-end testing with deterministic execution verified**
- [ ] **Performance testing shows acceptable latency**
- [ ] **Security testing validates fraud proof mechanisms**
- [ ] **Integration testing with ioID-SDK devices successful**

## 🔄 **Rollback Strategy**

### **Dual-Mode Operation**

```rust
// Support both MVP and Cartesi during transition
#[derive(Debug, Deserialize)]
pub enum ExecutionMode {
    MVP,
    Cartesi,
    DualMode { primary: Box<ExecutionMode> },
}

pub struct DualModeProcessor {
    mvp_processor: MVPProcessor,
    cartesi_processor: CartesiProcessor,
    mode: ExecutionMode,
}

impl DualModeProcessor {
    pub async fn process_data(&self, data: &IoTData) -> Result<ProcessingResult> {
        match &self.mode {
            ExecutionMode::MVP => self.mvp_processor.process(data).await,
            ExecutionMode::Cartesi => self.cartesi_processor.process(data).await,
            ExecutionMode::DualMode { primary } => {
                // Run both systems and compare results
                let (mvp_result, cartesi_result) = tokio::join!(
                    self.mvp_processor.process(data),
                    self.cartesi_processor.process(data)
                );
                
                match primary.as_ref() {
                    ExecutionMode::MVP => mvp_result.or(cartesi_result),
                    ExecutionMode::Cartesi => cartesi_result.or(mvp_result),
                    _ => unreachable!(),
                }
            }
        }
    }
}
```

## 📈 **Migration Success Criteria**

### **Technical Success Criteria**

- **Deterministic Execution**: 100% reproducible results across executions
- **Performance**: Maintain MVP performance levels
- **Fraud Detection**: Successful dispute resolution in test scenarios
- **Data Integrity**: Zero data loss during migration

### **Functional Success Criteria**

- **Feature Parity**: All MVP functionality working in Cartesi
- **Security Compliance**: Enhanced security through deterministic execution
- **Scalability**: System handles expected device load
- **Integration**: KC-Chain settlement working correctly

### **Operational Success Criteria**

- **Deployment**: Cartesi infrastructure deployed and operational
- **Monitoring**: Full observability into Cartesi VM operations
- **Documentation**: Complete technical documentation for operations
- **Team Knowledge**: Development team fully trained on Cartesi operations

This integration plan provides a comprehensive technical strategy for migrating from MVP to full Cartesi integration, ensuring all components work together in the final architecture.
