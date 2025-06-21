export interface GeneratePGPKeysResponse {
    public_key: string;
    private_key: string;
}

export interface Key{
    key_id: string,
    nickname: string,
    metadata?: string,
    is_me: boolean,
    private_key?: string,
    public_key: string,
    created_at: Date,
   
}

