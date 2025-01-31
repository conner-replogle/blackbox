export interface GeneratePGPKeysResponse {
    public_key: string;
    private_key: string;
}


export interface PrivateKey {
    key_id: string,
    nickname: string,
    metadata?: string,
    private_key: string,
    public_key_id: string,
    created_at: Date,
}

export interface PublicKey{
    key_id: string,
    nickname: string,
    metadata?: string,
    is_me: boolean,
    public_key: string,
    created_at: Date
}

