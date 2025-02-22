from rest_framework import serializers
from .models import Asset, Account, Transaction

class AccountSerializer(serializers.ModelSerializer):    
    class Meta:
        model = Account
        fields = ['username', 'balance']


class AssetSerializer(serializers.ModelSerializer):
    class Meta:
        model = Asset
        fields = ['account', 'asset_type', 'balance']

class TransactionSerializer(serializers.ModelSerializer):
    class Meta:
        model = Transaction
        fields = [
            'account', 'asset', 'transaction_type', 'from_account', 'to_account'
        ]

    def validate(self, data):
        transaction_type = data.get('transaction_type')
        from_account = data.get('from_account')
        to_account = data.get('to_account')
        asset = data.get('asset')

        if transaction_type == Transaction.TransactionType.TRANSFER:
            if not from_account or not to_account:
                raise serializers.ValidationError("Both from_account and to_account are required for a Transfer transaction.")
            if from_account == to_account:
                raise serializers.ValidationError("from_account and to_account cannot be the same account.")

        elif transaction_type in [Transaction.TransactionType.INCOME, Transaction.TransactionType.EXPENSE]:
            if from_account or to_account:
                raise serializers.ValidationError("from_account and to_account should only be set for Transfer transactions.")
        # Validate Asset existence
        if asset and not Asset.objects.filter(id=asset.id).exists():
            raise serializers.ValidationError("The specified asset does not exist.")

        return data
