# Deploying the Streamlit Demo

## Streamlit Community Cloud

1. Push the `streamlit_app/` directory to a GitHub repository.
2. Go to [share.streamlit.io](https://share.streamlit.io) and connect the repo.
3. Set the main file path to `streamlit_app/app.py`.
4. No secrets or environment variables are required.

### Resource Limits (Free Tier)
- Memory: 1 GB RAM
- Storage: 10 GB
- Idle timeout: ~15 minutes

### Privacy Note
This demo uses only synthetic sample data. No user data is collected, stored,
or transmitted. No API keys or authentication are required.

## Local Deployment

```text
pip install -r streamlit_app/requirements.txt
streamlit run streamlit_app/app.py
```

## Claim Boundary

This app is a **synthetic-data demonstrator only**. It does not:
- Call live provider APIs
- Require API keys
- Mutate repository state
- Handle real user data
