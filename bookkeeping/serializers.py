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
            'account', 'asset', 'transaction_type', 'from_account', 'to_account', 'amount'
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

    def create(self, validated_data):
        account = validated_data.get('account')
        asset = validated_data.get('asset')
        asset_type = self.initial_data.get('asset_type')
        
        if not Account.objects.filter(username=account.username).exists():
            account, _ = Account.objects.get_or_create(username=account.username, defaults={"balance": 0})

        if asset is None and asset_type:
            asset, _ = Asset.objects.get_or_create(account=account, asset_type=asset_type, defaults={"balance": 0})

        validated_data['account'] = account
        validated_data['asset'] = asset

        return super().create(validated_data)
