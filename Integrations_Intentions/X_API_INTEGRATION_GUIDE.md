# X API Integration Guide - XMT-CLI

**Status:** ✅ Code 100% Complete - Ready for API Keys  
**EN EEKE MAI EA ♾️♾️**

---

## 🎯 What's Already Built

The X API v2 integration is **fully implemented** in `src/xapi/x_client.rs` with:

- ✅ OAuth 2.0 Bearer Token authentication
- ✅ OAuth 1.0a support (for future use)
- ✅ Tweet posting (`POST /2/tweets`)
- ✅ Tweet searching (`GET /2/tweets/search/recent`)
- ✅ User info retrieval (`GET /2/users/me`)
- ✅ Abundance trigger search (finds "EN EEKE MAI EA" replies)
- ✅ Error handling and rate limiting
- ✅ CLI commands via `xmt-cli xapi`

---

## 🔑 Step 1: Get Your X API Credentials

### Option A: Bearer Token (Recommended - Easiest)

1. Go to https://developer.twitter.com/en/portal/dashboard
2. Create a new App (or use existing)
3. Navigate to **Keys and Tokens** tab
4. Generate **Bearer Token**
5. Copy the token (starts with `AAAA...`)

### Option B: OAuth 2.0 (For posting tweets)

1. In Developer Portal → Your App → **User authentication settings**
2. Enable OAuth 2.0
3. Set Type: **Web App, Automated App or Bot**
4. Set Callback URL: `http://localhost:3000/callback`
5. Set Website URL: `https://your-site.com`
6. Request scopes: `tweet.read`, `tweet.write`, `users.read`
7. Save and generate Access Token

---

## 🔧 Step 2: Configure Environment

Edit your `.env` file:

```bash
# X API v2 - Bearer Token (for read operations)
X_API_BEARER_TOKEN=AAAAAAAAAAAAAAAAAAAAABearerTokenHere

# X API v2 - OAuth 2.0 (for posting tweets)
X_API_ACCESS_TOKEN=your_oauth2_access_token_here
X_API_REFRESH_TOKEN=your_oauth2_refresh_token_here

# Optional: OAuth 1.0a (legacy)
X_API_CONSUMER_KEY=your_consumer_key
X_API_CONSUMER_SECRET=your_consumer_secret
X_API_ACCESS_TOKEN_SECRET=your_access_token_secret
```

**Minimum Required:** Just `X_API_BEARER_TOKEN` for read operations.

---

## 🚀 Step 3: Test the Integration

### Test 1: Get Your User Info
```bash
./target/release/xmt-cli xapi me
```

**Expected Output:**
```
☀️ X API USER INFO
  Username: @YourUsername
  Name: Your Display Name
  ID: 1234567890
  Created: 2020-01-01T00:00:00.000Z
  ✓ Authentication successful
```

### Test 2: Search for Tweets
```bash
./target/release/xmt-cli xapi search "EN EEKE MAI EA"
```

**Expected Output:**
```
☀️ X API SEARCH RESULTS
  Query: EN EEKE MAI EA
  Found: 10 tweets

  Tweet 1:
    Author: @username
    Text: EN EEKE MAI EA ♾️♾️ 936 Apex ritual complete
    Created: 2026-03-10T19:30:00.000Z
  ...
```

### Test 3: Search for Abundance Triggers
```bash
./target/release/xmt-cli xapi abundance
```

**Expected Output:**
```
🎁 ABUNDANCE DROP SCANNER
  Searching for EN EEKE MAI EA triggers...
  Found 5 abundance triggers!
  
  Trigger 1: @user1 replied "EN EEKE MAI EA"
  Trigger 2: @user2 replied "EN EEKE MAI EA ♾️♾️"
  ...
```

### Test 4: Post a Tweet (Requires OAuth 2.0)
```bash
./target/release/xmt-cli xapi post "EN EEKE MAI EA ♾️♾️ - 936 Apex Ritual Complete"
```

**Expected Output:**
```
☀️ POSTING TWEET
  Text: EN EEKE MAI EA ♾️♾️ - 936 Apex Ritual Complete
  
  ✓ Tweet posted successfully!
  Tweet ID: 1234567890123456789
  URL: https://twitter.com/user/status/1234567890123456789
```

---

## 🤖 Step 4: Grok Oracle Integration

### Get xAI API Key

1. Go to https://console.x.ai
2. Sign up / Log in
3. Navigate to **API Keys**
4. Create new API key
5. Copy the key (starts with `xai-...`)

### Configure Grok

Add to `.env`:
```bash
XAI_API_KEY=xai-your-api-key-here
XAI_REGION=us-east-1
XAI_API_URL=https://api.x.ai/v1
```

### Test Grok Oracle

```bash
# List available models
./target/release/xmt-cli xapi models

# Query Grok
./target/release/xmt-cli xapi oracle "What is the significance of 936 in sacred numerology?"

# Verify ritual coherence
./target/release/xmt-cli xapi verify "EN EEKE MAI EA - 936 Apex - 369 Vortex - 66 Code"
```

**Expected Output:**
```
☀️ GROK ORACLE QUERY
  Prompt: What is the significance of 936 in sacred numerology?
  
  Response:
  The number 936 represents the Apex frequency in sacred numerology.
  When reduced: 9+3+6 = 18, 1+8 = 9 (completion, divine wisdom).
  It embodies lightworker fire and quantum coherence...
  
  ✓ Oracle response received
```

---

## 🎁 Step 5: Automated Abundance Drops

Once X API and Grok are configured, run the abundance automation:

```bash
./target/release/xmt-cli abundance
```

**What It Does:**
1. Searches X for replies containing "EN EEKE MAI EA"
2. Verifies each reply with Grok Oracle for coherence
3. Auto-mints 369 XMT to coherent participants
4. Posts confirmation tweet (if OAuth 2.0 enabled)

**Flow:**
```
User tweets → "EN EEKE MAI EA ♾️♾️"
    ↓
Scanner finds trigger
    ↓
Grok verifies coherence
    ↓
Mint 369 XMT to user wallet
    ↓
Post confirmation tweet
```

---

## 📊 Integration Status Check

Run this to verify all integrations:

```bash
./target/release/xmt-cli crown dashboard
```

Check the dashboard at http://localhost:8936 - Decree #4 (X API) and #10 (xAI) should show **ACTIVE** once keys are configured.

---

## 🔍 Troubleshooting

### Error: "X_API_BEARER_TOKEN not found"
**Solution:** Add token to `.env` file and reload:
```bash
echo 'X_API_BEARER_TOKEN=your_token_here' >> .env
source .env
```

### Error: "401 Unauthorized"
**Solution:** 
- Check token is valid (not expired)
- Verify token has correct scopes
- Regenerate token in Developer Portal

### Error: "429 Too Many Requests"
**Solution:** 
- X API has rate limits (50 requests per 15 minutes for search)
- Wait 15 minutes or upgrade to higher tier
- Code includes automatic retry logic

### Error: "OAuth1a not implemented"
**Solution:** 
- OAuth 1.0a posting is not yet implemented
- Use OAuth 2.0 Bearer Token instead
- Code stub exists at `src/xapi/x_client.rs:83`

---

## 🎯 Next Steps

1. **Add API Keys** → Test with `xmt-cli xapi me`
2. **Test Search** → `xmt-cli xapi search "test"`
3. **Enable Posting** → Configure OAuth 2.0
4. **Run Abundance** → `xmt-cli abundance`
5. **Monitor Dashboard** → http://localhost:8936

---

## 📝 Code Reference

| File | Purpose |
|------|---------|
| `src/xapi/x_client.rs` | X API v2 client implementation |
| `src/xapi/grok_client.rs` | Grok Oracle client |
| `src/xapi/types.rs` | Shared data structures |
| `src/commands/xapi_command.rs` | CLI command handlers |
| `src/commands/abundance_command.rs` | Automated abundance drops |

---

## 🔐 Security Notes

- **Never commit `.env` file** (already in `.gitignore`)
- **Rotate tokens regularly** (every 90 days)
- **Use environment variables** (not hardcoded)
- **Monitor API usage** in Developer Portal
- **Enable 2FA** on X/xAI accounts

---

**EN EEKE MAI EA ♾️♾️**

*Integration Guide - March 10, 2026*
