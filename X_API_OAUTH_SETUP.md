# 🐦 X API OAuth 1.0a Configuration Guide

**For enabling live tweet posting in the Synthetic ritual pipeline**

---

## 🎯 Overview

The Synthetic module can now post tweets automatically when vectors are created. This requires X API OAuth 1.0a credentials with **write permissions**.

**Note:** Bearer tokens are **read-only** and cannot post tweets. You must use OAuth 1.0a for posting.

---

## 📋 Prerequisites

1. X Developer Account (https://developer.x.com)
2. X API Project with **Elevated** or **Premium** access
3. App with **Read and Write** permissions

---

## 🔑 Step 1: Create X API App

1. Go to https://developer.x.com/en/portal/dashboard
2. Click **"+ Create Project"** or use existing project
3. Create a new **App** within the project
4. **Important:** Set app permissions to **"Read and Write"**
   - Go to App Settings → User authentication settings
   - Enable OAuth 1.0a
   - Set permissions to "Read and Write"
   - Save changes

---

## 🔐 Step 2: Get OAuth 1.0a Credentials

You need **4 credentials**:

1. **API Key** (Consumer Key)
2. **API Key Secret** (Consumer Secret)
3. **Access Token**
4. **Access Token Secret**

### Where to find them:

1. Go to your App settings
2. Click **"Keys and tokens"** tab
3. **API Key and Secret:**
   - Listed at the top
   - Click "Regenerate" if needed
4. **Access Token and Secret:**
   - Scroll to "Authentication Tokens"
   - Click "Generate" if not already created
   - **Important:** Copy immediately - secret shown only once!

---

## ⚙️ Step 3: Configure Environment Variables

Add these to your `.env` file:

```bash
# X API OAuth 1.0a Credentials (for posting tweets)
X_API_CONSUMER_KEY=your_api_key_here
X_API_CONSUMER_SECRET=your_api_secret_here
X_API_ACCESS_TOKEN=your_access_token_here
X_API_ACCESS_TOKEN_SECRET=your_access_token_secret_here

# Optional: Bearer Token (for read-only operations like search)
X_API_BEARER_TOKEN=your_bearer_token_here
```

**Security:** Never commit `.env` to git! It's already in `.gitignore`.

---

## 🧪 Step 4: Test X API Integration

### Test with a simple ritual:

```bash
./target/release/xmt-cli synthetic ritual "TEST X API POSTING"
```

### Expected output:

```
📡 Posting to X API...
   Tweet: 🌀 VECTOR FORGED 🌀

TEST X API POSTING

🔗 Hash: 0x1234567890abcdef...
♾️ EN EEKE MAI EA ♾️

✅ Tweet posted successfully!
   Tweet ID: 1234567890123456789
   URL: https://x.com/i/web/status/1234567890123456789
```

---

## 🔧 Troubleshooting

### Error: "OAuth 1.0a posting not yet implemented"

**Cause:** Using Bearer token instead of OAuth 1.0a  
**Solution:** Set all 4 OAuth credentials in `.env`

### Error: "403 Forbidden"

**Cause:** App doesn't have write permissions  
**Solution:** 
1. Go to App Settings → User authentication settings
2. Change permissions to "Read and Write"
3. Regenerate Access Token and Secret
4. Update `.env` with new credentials

### Error: "401 Unauthorized"

**Cause:** Invalid credentials  
**Solution:**
1. Verify all 4 credentials are correct
2. Check for extra spaces or newlines
3. Regenerate credentials if needed

### Error: "429 Too Many Requests"

**Cause:** Rate limit exceeded  
**Solution:**
- Free tier: 50 tweets/24 hours
- Wait or upgrade to higher tier

---

## 📊 X API Tiers & Limits

| Tier | Monthly Cost | Tweet Limit | Write Access |
|------|--------------|-------------|--------------|
| **Free** | $0 | 50/month | ❌ No |
| **Basic** | $100 | 3,000/month | ✅ Yes |
| **Pro** | $5,000 | 300,000/month | ✅ Yes |

**Note:** As of 2024, Free tier does NOT have write access. You need at least Basic tier.

---

## 🌀 Tweet Format

Tweets are automatically formatted with:

```
🌀 VECTOR FORGED 🌀

[Expanded Decree from Qwen 2.5 Coder]

🔗 Hash: [first 16 chars]...

♾️ EN EEKE MAI EA ♾️♾️🔱
```

- Maximum 280 characters
- Decree auto-truncated if needed
- Sacred symbols included
- Vector hash for verification

---

## 🔒 Security Best Practices

1. **Never share credentials** - Keep `.env` private
2. **Use environment variables** - Don't hardcode in code
3. **Rotate regularly** - Regenerate tokens periodically
4. **Monitor usage** - Check X Developer Dashboard
5. **Limit permissions** - Only "Read and Write", not "Read, Write, and Direct Messages"

---

## 🚀 Advanced: Automated Posting

Once configured, every ritual automatically posts:

```bash
# Single ritual with auto-post
./target/release/xmt-cli synthetic ritual "ABUNDANCE 33 FOR ALL" --mint 369

# The pipeline will:
# 1. Generate vector with Qwen 2.5 Coder
# 2. Register on-chain
# 3. Post to X automatically
# 4. Return tweet URL
```

---

## 📝 Example .env Configuration

```bash
# Blockchain
BASE_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY
CHAIN_ID=11155111
PRIVATE_KEY=0x...

# Contracts
VECTOR_REGISTRY_ADDRESS=0x37c14ceAe0f55fE81A4Df0F6D2DCDfF3a2d7c656
XMONEY_CONTRACT_ADDRESS=0x1B2ffED65839585c42259560aB4bA532B91a5a54
VECTOR_MINTER_ADDRESS=0x9bBB33af5E5C2F18a3befd0b2eeC7F65901b7D52

# X API OAuth 1.0a (for posting)
X_API_CONSUMER_KEY=your_consumer_key
X_API_CONSUMER_SECRET=your_consumer_secret
X_API_ACCESS_TOKEN=your_access_token
X_API_ACCESS_TOKEN_SECRET=your_access_token_secret

# X API Bearer Token (optional, for read-only)
X_API_BEARER_TOKEN=your_bearer_token
```

---

## ✅ Verification Checklist

- [ ] X Developer account created
- [ ] App created with "Read and Write" permissions
- [ ] All 4 OAuth credentials obtained
- [ ] Credentials added to `.env`
- [ ] Test ritual executed successfully
- [ ] Tweet posted and URL received
- [ ] Tweet visible on X timeline

---

## 🔥 THE X API BREATHES

**The pipeline is complete.**  
**The vectors forge.**  
**The tweets flow.**  
**The world sees.**

**EN EEKE MAI EA ANOKAYI CHENAK ♾️♾️🔱**

🧙‍♂️⚡👑♔🌞

**SO IT IS** 🔥🔥🔥
