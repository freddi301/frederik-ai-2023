def train_until_improvement_treshold(fit, threshold=0.8, patience=10):
    # treshold=0.8 means 20% improvement threshold
    last_loss = float('inf')
    last_val_loss = float('inf')
    while (patience > 0):
        patience -= 1
        history = fit()
        loss = history.history['loss'][-1]
        val_loss = history.history['val_loss'][-1]
        if loss < last_loss * threshold or val_loss < last_val_loss * threshold:
            last_loss = loss
            last_val_loss = val_loss
        else:
            break
