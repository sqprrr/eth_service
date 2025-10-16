   git clone https://github.com/<твой_ник>/eth_service.git
   cd eth_service

Create .env:

DATABASE_URL=postgres://postgres:postgres@db:5432/eth_db
RPC_URL=https://mainnet.infura.io/v3/<your_api_key>

Docker:

docker compose up --build
