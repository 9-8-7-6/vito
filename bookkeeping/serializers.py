from rest_framework import serializers
from django.contrib.auth.models import User
from .models import Asset, Account, Transaction, User

class UserSerializer(serializers.ModelSerializer):
    class Meta:
        model = User
        fields = ['name']


class AccountSerializer(serializers.ModelSerializer):    
    class Meta:
        model = Account
        fields = ['user', 'balance']


class AssetSerializer(serializers.ModelSerializer):
    class Meta:
        model = Asset
        fields = ['user', 'type', 'balance']

class TransactionSerializer(serializers.ModelSerializer):
    class Meta:
        model = Transaction
        fields = [
            'account', 'asset', 'transaction_type', 'from_account', 'to_account',  'amount'
        ]

    def validate(self, data):
        transaction_type = data.get('transaction_type')
        from_account = data.get('from_account')
        to_account = data.get('to_account')
        asset = data.get('asset')

        if transaction_type == Transaction.TransactionType.TRANSFER:
            if not from_account or not to_account:
                raise serializers.ValidationError("Both from_account and to_account are required for a Transfer transaction.")
        else:
            if from_account or to_account:
                raise serializers.ValidationError("from_account and to_account should be null if the transfer type is Transfer.")

        # make sure the asset exists already
        if asset and not Asset.objects.filter(id=asset.id).exists():
            raise serializers.ValidationError("The specified asset does not exist.")

        return data
