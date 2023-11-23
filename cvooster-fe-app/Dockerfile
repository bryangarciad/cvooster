# Use a Node.js base image with a specified version
FROM node:20-alpine

# Set the working directory inside the container
WORKDIR /app

# Copy package.json and package-lock.json to the container
COPY package.json package-lock.json ./

# Install dependencies
RUN npm install

# Copy the entire Next.js app directory into the container
COPY . .

# Build the Next.js app for production
RUN npm run build

# Expose the default Next.js port (change it if your app uses a different port)
EXPOSE 3000

# Set the command to start the Next.js app
CMD ["npm", "start"]
