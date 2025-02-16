from django.db import models
from django.contrib.auth.models import User
from decimal import Decimal
# Create your models here.

class Asset(models.Model):
    id = models.AutoField(primary_key=True)
    user = models.ForeignKey(User, on_delete=models.CASCADE)
    type = models.CharField(max_length=255)
    balance = models.DecimalField(max_digits=12, decimal_places=2, default=Decimal("0.00"))
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)
    
    class Meta:
        db_table = "asset"
        unique_together = ('user', 'type')
        
    def __str__(self):
        return self.type

class Account(models.Model):
    user = models.OneToOneField(User, on_delete=models.CASCADE, primary_key=True)
    balance = models.DecimalField(max_digits=12, decimal_places=2, default=Decimal("0.00"))
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    class Meta:
        db_table = "account"
        ordering = ['-created_at']
    
    def __str__(self):
        return f"{self.user.username} - Account"

class Transaction(models.Model):
    class TransactionType(models.IntegerChoices):
        INCOME = 1, "Income"
        EXPENSE = 2, "Expense"
        TRANSFER = 3, "Transfer"

    account = models.ForeignKey(Account, on_delete=models.CASCADE, related_name="transactions")
    asset = models.ForeignKey(Asset, on_delete=models.CASCADE)
    transaction_type = models.PositiveSmallIntegerField(choices=TransactionType.choices)
    amount = models.DecimalField(max_digits=12, decimal_places=2)
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)
    
    class Meta:
        db_table = "transaction"
        ordering = ['-created_at']

    def __str__(self):
        return f"{self.account} - {self.amount} ({self.asset})"
