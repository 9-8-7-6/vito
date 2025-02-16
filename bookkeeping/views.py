from django.shortcuts import render
from .serializers import AccountSerializer, AssetSerializer, TransactionSerializer, UserSerializer
from .models import Account, Asset, Transaction, User
from rest_framework import viewsets

class UserViewSet(viewsets.ModelViewSet):
    serializer_class = UserSerializer
    queryset = User.objects.all()

class AccountViewSet(viewsets.ModelViewSet):
    serializer_class = AccountSerializer
    queryset = Account.objects.all()

class AssetViewSet(viewsets.ModelViewSet):
    serializer_class = AssetSerializer
    queryset = Asset.objects.all()
    
class TransactionViewSet(viewsets.ModelViewSet):
    serializer_class = TransactionSerializer
    queryset = Transaction.objects.all()
