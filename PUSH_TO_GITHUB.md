# How to Push to GitHub

## Quick Steps

1. **Create a new repository on GitHub:**
   - Go to: <https://github.com/new>
   - Repository name: `rust-torrent-client` (or any name you like)
   - Description: "A BitTorrent client written in Rust"
   - Keep it **Public** or **Private** (your choice)
   - **Don't** initialize with README, .gitignore, or license (we already have these)
   - Click "Create repository"

2. **Connect and push your code:**

After creating the repo on GitHub, run these commands:

```bash
# Add GitHub remote (replace YOUR_USERNAME with your GitHub username)
git remote add origin https://github.com/YOUR_USERNAME/rust-torrent-client.git

# Push to GitHub
git branch -M main
git push -u origin main
```

## Full Example

If your GitHub username is `windxebec`, you would run:

```bash
cd "c:\Users\wind xebec\OneDrive\Desktop\abc"

git remote add origin https://github.com/windxebec/rust-torrent-client.git
git branch -M main
git push -u origin main
```

## What I've Already Done

✅ Created `.gitignore` file
✅ Initialized Git repository (`git init`)
✅ Added all files (`git add .`)
✅ Created first commit with message: "Initial commit: Rust BitTorrent client implementation"

## Next: You Need To

1. Go to <https://github.com/new> and create the repository
2. Copy the commands shown on GitHub (they'll look like above)
3. Run them in your terminal

That's it! Your code will be on GitHub! 🚀
