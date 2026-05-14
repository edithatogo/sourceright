# Streamlit App Publication And Hardening Spec

## Goal

Make the Streamlit demo deployable and hardened as a public demonstration
surface while keeping it synthetic-data-only and separate from production
service claims.

## Contract

- Server startup and render smoke are reproducible.
- Sample data is synthetic and documented.
- Optional Streamlit Community Cloud deployment has secrets, resource, and
  privacy notes.
- The app does not call live providers by default.
