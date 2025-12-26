/**
 * Main Application Entry Point
 * 
 * This file initializes the application, sets up middleware,
 * configures routes, and starts the HTTP server.
 */

import express, { Application, Request, Response, NextFunction } from 'express';
import { config } from './config';
import { authRouter } from './routes/auth';
import { userRouter } from './routes/users';
import { productRouter } from './routes/products';
import { orderRouter } from './routes/orders';
import { logger } from './utils/logger';
import { errorHandler } from './middleware/errorHandler';
import { requestLogger } from './middleware/requestLogger';

const app: Application = express();

// Middleware configuration
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Request logging
app.use(requestLogger);

// Health check endpoint
app.get('/health', (req: Request, res: Response) => {
    res.json({
        status: 'healthy',
        timestamp: new Date().toISOString(),
        version: config.version,
    });
});

// API routes
app.use('/api/auth', authRouter);
app.use('/api/users', userRouter);
app.use('/api/products', productRouter);
app.use('/api/orders', orderRouter);

// 404 handler
app.use((req: Request, res: Response) => {
    res.status(404).json({
        error: 'Not Found',
        message: `Route ${req.method} ${req.path} not found`,
    });
});

// Error handling middleware
app.use(errorHandler);

// Start server
const PORT = config.port || 3000;
const HOST = config.host || '0.0.0.0';

app.listen(PORT, HOST, () => {
    logger.info(`Server started on ${HOST}:${PORT}`);
    logger.info(`Environment: ${config.environment}`);
    logger.info(`API Version: ${config.apiVersion}`);
});

// Graceful shutdown handling
process.on('SIGTERM', () => {
    logger.info('SIGTERM received. Shutting down gracefully...');
    process.exit(0);
});

process.on('SIGINT', () => {
    logger.info('SIGINT received. Shutting down gracefully...');
    process.exit(0);
});

export default app;
