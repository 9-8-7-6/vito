from django.db import models
from decimal import Decimal


class Account(models.Model):
    username = models.CharField(max_length=255, primary_key=True)
    balance = models.DecimalField(max_digits=12, decimal_places=2, default=Decimal("0.00"))
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    class Meta:
        db_table = "account"
        ordering = ['-created_at']
    
    def __str__(self):
        return f"{self.username} - Account"

class Asset(models.Model):
    id = models.AutoField(primary_key=True)
    account = models.ForeignKey(Account, on_delete=models.CASCADE)
    asset_type = models.CharField(max_length=255)
    balance = models.DecimalField(max_digits=12, decimal_places=2, default=Decimal("0.00"))
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)
    
    class Meta:
        db_table = "asset"
        constraints = [
            models.UniqueConstraint(fields=['account', 'asset_type'], name='unique_account_asset_asset_type')
        ]
        
    def __str__(self):
        return self.asset_type

class Transaction(models.Model):
    class TransactionType(models.IntegerChoices):
        INCOME = 1, "Income"
        EXPENSE = 2, "Expense"
        TRANSFER = 3, "Transfer"

    account = models.ForeignKey(Account, on_delete=models.CASCADE, related_name="transactions")
    asset = models.ForeignKey(Asset, on_delete=models.CASCADE, null=True, blank=True)
    transaction_type = models.PositiveSmallIntegerField(choices=TransactionType.choices)
    amount = models.DecimalField(max_digits=12, decimal_places=2)
    from_account = models.ForeignKey(
        Account, on_delete=models.CASCADE, related_name="transfers_out", null=True, blank=True
    )
    to_account = models.ForeignKey(
        Account, on_delete=models.CASCADE, related_name="transfers_in", null=True, blank=True
    )
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)
    
    class Meta:
        db_table = "transaction"
        ordering = ['-created_at']

    def __str__(self):
        return f"{self.account} - {self.amount} ({self.asset})"
