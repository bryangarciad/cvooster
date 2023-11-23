# Use the official Python base image from Docker Hub
FROM python:3.9

# Set working directory inside the container
WORKDIR /app

# Copy the requirements file and install dependencies
COPY requirements.txt .

RUN pip install -r requirements.txt

# Copy all project files to the working directory
COPY . .

# Command to run the Python application
CMD ["python", "app.py"]
