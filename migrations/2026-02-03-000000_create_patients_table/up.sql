-- Create patients table for HPorterly (generic dataset support)
CREATE TABLE patients (
    id VARCHAR(50) PRIMARY KEY,  -- Format: IPP-XXXXXX
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    age INTEGER,
    gender CHAR(1) CHECK (gender IN ('M', 'F', 'O')),
    service VARCHAR(100),
    room VARCHAR(50),
    building VARCHAR(50),
    floor VARCHAR(50),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for efficient patient search
CREATE INDEX idx_patients_name ON patients(last_name, first_name);
CREATE INDEX idx_patients_ipp ON patients(id);
CREATE INDEX idx_patients_service ON patients(service);
